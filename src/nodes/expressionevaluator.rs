use std::sync::Mutex;

use super::{
    node::Node,
    traits::{ConstVisitable, NodeConstVisitor},
};

use crate::utils::errors::{Result, ScriptingError};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Number(f64),
    Null,
}

//type MarketData = Vec<f64>;
#[allow(unused)]
pub struct ExpressionEvaluator {
    variables: Mutex<Vec<Value>>,
    digit_stack: Mutex<Vec<f64>>,
    boolean_stack: Mutex<Vec<bool>>,
    is_lhs_variable: Mutex<bool>,
    lhs_variable: Mutex<Option<Box<Node>>>,
    current_event: Option<usize>,
}

impl ExpressionEvaluator {
    pub fn new() -> Self {
        ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        }
    }

    pub fn with_variables(self, n: usize) -> Self {
        self.variables.lock().unwrap().resize(n, Value::Null);
        self
    }

    pub fn variables(&self) -> Vec<Value> {
        self.variables.lock().unwrap().clone()
    }

    pub fn digit_stack(&self) -> Vec<f64> {
        self.digit_stack.lock().unwrap().clone()
    }

    pub fn boolean_stack(&self) -> Vec<bool> {
        self.boolean_stack.lock().unwrap().clone()
    }
}

impl NodeConstVisitor for ExpressionEvaluator {
    type Output = Result<()>;
    fn const_visit(&self, node: Box<Node>) -> Self::Output {
        let eval: Result<()> = match node.as_ref() {
            Node::Base(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;
                Ok(())
            }
            Node::Variable(_, name, index) => {
                if *self.is_lhs_variable.lock().unwrap() {
                    *self.lhs_variable.lock().unwrap() = Some(node.clone());
                    Ok(())
                } else {
                    match index.get() {
                        None => {
                            return Err(ScriptingError::EvaluationError(format!(
                                "Variable {} not indexed",
                                name
                            )))
                        }
                        Some(id) => {
                            // let value = self.variables.lock().unwrap()[*id];
                            // self.digit_stack.lock().unwrap().push(value);

                            //let value = self.variables.lock().unwrap().get(*id).unwrap();
                            let vars = self.variables.lock().unwrap();
                            let value = vars.get(*id).unwrap();
                            match value {
                                Value::Number(v) => self.digit_stack.lock().unwrap().push(*v),
                                Value::Bool(v) => self.boolean_stack.lock().unwrap().push(*v),
                                Value::Null => {
                                    return Err(ScriptingError::EvaluationError(format!(
                                        "Variable {} not initialized",
                                        name
                                    )))
                                }
                            }
                            Ok(())
                        }
                    }
                }
            }

            Node::Constant(value) => {
                self.digit_stack.lock().unwrap().push(*value);
                Ok(())
            }
            Node::Add(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(left + right);
                Ok(())
            }
            Node::Subtract(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(left - right);
                Ok(())
            }
            Node::Multiply(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(left * right);
                Ok(())
            }
            Node::Divide(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(left / right);
                Ok(())
            }
            Node::Assign(children) => {
                *self.is_lhs_variable.lock().unwrap() = true;
                children.get(0).unwrap().const_accept(self);

                *self.is_lhs_variable.lock().unwrap() = false;
                children.get(1).unwrap().const_accept(self);

                let v = self.lhs_variable.lock().unwrap().clone().unwrap();
                let variable = v.as_ref();
                match variable {
                    Node::Variable(_, name, index) => match index.get() {
                        None => {
                            return Err(ScriptingError::EvaluationError(format!(
                                "Variable {} not indexed",
                                name
                            )))
                        }
                        Some(id) => {
                            // let value = self.digit_stack.lock().unwrap().pop().unwrap();
                            // self.variables.lock().unwrap()[*id] = value

                            let mut variables = self.variables.lock().unwrap();
                            if !self.boolean_stack.lock().unwrap().is_empty() {
                                // Pop from boolean stack and store the boolean value
                                let value = self.boolean_stack.lock().unwrap().pop().unwrap();
                                variables[*id] = Value::Bool(value);

                                Ok(())
                            } else {
                                // Pop from digit stack and store the numeric value
                                let value = self.digit_stack.lock().unwrap().pop().unwrap();
                                variables[*id] = Value::Number(value);

                                Ok(())
                            }
                        }
                    },
                    _ => {
                        return Err(ScriptingError::EvaluationError(
                            "Invalid variable assignment".to_string(),
                        ))
                    }
                }
            }
            Node::NotEqual(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.boolean_stack
                    .lock()
                    .unwrap()
                    .push((right - left).abs() >= f64::EPSILON);

                Ok(())
            }
            Node::And(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.boolean_stack.lock().unwrap().pop().unwrap();
                let left = self.boolean_stack.lock().unwrap().pop().unwrap();
                self.boolean_stack.lock().unwrap().push(left && right);

                Ok(())
            }
            Node::Or(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.boolean_stack.lock().unwrap().pop().unwrap();
                let left = self.boolean_stack.lock().unwrap().pop().unwrap();
                self.boolean_stack.lock().unwrap().push(left || right);

                Ok(())
            }
            Node::Not(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let value = self.boolean_stack.lock().unwrap().pop().unwrap();
                self.boolean_stack.lock().unwrap().push(!value);

                Ok(())
            }
            Node::Superior(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.boolean_stack.lock().unwrap().push(left > right);

                Ok(())
            }
            Node::Inferior(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.boolean_stack.lock().unwrap().push(left < right);

                Ok(())
            }
            Node::SuperiorOrEqual(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.boolean_stack.lock().unwrap().push(left >= right);

                Ok(())
            }
            Node::InferiorOrEqual(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.boolean_stack.lock().unwrap().push(left <= right);

                Ok(())
            }
            Node::True => {
                self.boolean_stack.lock().unwrap().push(true);

                Ok(())
            }

            Node::False => {
                self.boolean_stack.lock().unwrap().push(false);

                Ok(())
            }
            Node::Equal(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();

                self.boolean_stack
                    .lock()
                    .unwrap()
                    .push((right - left).abs() < f64::EPSILON);

                Ok(())
            }
            Node::UnaryPlus(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                Ok(())
            }
            Node::UnaryMinus(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let top = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(-top);

                Ok(())
            }
            Node::Min(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(left.min(right));

                Ok(())
            }
            Node::Max(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(left.max(right));

                Ok(())
            }
            Node::Pow(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let right = self.digit_stack.lock().unwrap().pop().unwrap();
                let left = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(left.powf(right));

                Ok(())
            }
            Node::Ln(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let top = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(top.ln());

                Ok(())
            }
            Node::Exp(children) => {
                children
                    .iter()
                    .try_for_each(|child| self.const_visit(child.clone()))?;

                let top = self.digit_stack.lock().unwrap().pop().unwrap();
                self.digit_stack.lock().unwrap().push(top.exp());

                Ok(())
            }
            Node::If(children, first_else) => {
                // Evaluate the condition
                children.get(0).unwrap().const_accept(self);
                // Pop the condition result
                let is_true = self.boolean_stack.lock().unwrap().pop().unwrap();

                // Find the first else node
                if is_true {
                    // then, the following expressions are either conditions or
                    // the logic block
                    let last_condition = if first_else.is_none() {
                        children.len()
                    } else {
                        first_else.unwrap()
                    };

                    // Evaluate the conditions
                    for i in 1..last_condition {
                        children.get(i).unwrap().const_accept(self);
                    }
                }
                // Evaluate the else block
                else if first_else.is_some() {
                    // the following conditions are the else block
                    for i in first_else.unwrap()..children.len() {
                        children.get(i).unwrap().const_accept(self);
                    }
                }
                Ok(())
            }
        };
        eval
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_add_node() {
        let mut base = Box::new(Node::new_base());
        let mut add = Box::new(Node::new_add());

        let c1 = Box::new(Node::new_constant(1.0));
        let c2 = Box::new(Node::new_constant(1.0));

        add.add_child(c1);
        add.add_child(c2);
        base.add_child(add);

        let evaluator = ExpressionEvaluator::new();
        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.digit_stack().pop().unwrap(), 2.0);
    }

    #[test]
    fn test_subtract_node() {
        let mut base = Box::new(Node::new_base());
        let mut subtract = Node::new_subtract();

        let c1 = Node::new_constant(1.0);
        let c2 = Node::new_constant(1.0);

        subtract.add_child(Box::new(c1));
        subtract.add_child(Box::new(c2));
        base.add_child(Box::new(subtract));

        let evaluator = ExpressionEvaluator::new();
        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.digit_stack().pop().unwrap(), 0.0);
    }

    #[test]
    fn test_multiply_node() {
        let mut base = Box::new(Node::new_base());
        let mut multiply = Node::new_multiply();

        let c1 = Node::new_constant(2.0);
        let c2 = Node::new_constant(2.0);

        multiply.add_child(Box::new(c1));
        multiply.add_child(Box::new(c2));
        base.add_child(Box::new(multiply));

        let evaluator = ExpressionEvaluator::new();
        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.digit_stack().pop().unwrap(), 4.0);
    }

    #[test]
    fn test_divide_node() {
        let mut base = Box::new(Node::new_base());
        let mut divide = Node::new_divide();

        let c1 = Node::new_constant(4.0);
        let c2 = Node::new_constant(2.0);

        divide.add_child(Box::new(c1));
        divide.add_child(Box::new(c2));
        base.add_child(Box::new(divide));

        let evaluator = ExpressionEvaluator::new();
        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.digit_stack().pop().unwrap(), 2.0);
    }

    #[test]
    fn test_variable_assign_node() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let v1 = Box::new(Node::new_variable_with_id("x".to_string(), 0));

        let mut assign = Box::new(Node::new_assign());
        assign.add_child(v1);
        assign.add_child(c1);

        base.add_child(assign);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(vec![Value::Null]),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.variables().pop().unwrap(), Value::Number(1.0));
    }

    #[test]
    fn test_assign_boolean() {
        let base = Box::new(Node::Base(vec![
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "x".to_string(), 0.into())),
                Box::new(Node::True),
            ])),
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "y".to_string(), 1.into())),
                Box::new(Node::False),
            ])),
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "z".to_string(), 2.into())),
                Box::new(Node::And(vec![
                    Box::new(Node::Variable(Vec::new(), "x".to_string(), 0.into())),
                    Box::new(Node::Variable(Vec::new(), "y".to_string(), 1.into())),
                ])),
            ])),
        ]));

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(vec![Value::Null, Value::Null, Value::Null]),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };
        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.variables().get(0).unwrap(), &Value::Bool(true));
        assert_eq!(evaluator.variables().get(1).unwrap(), &Value::Bool(false));
        assert_eq!(evaluator.variables().get(2).unwrap(), &Value::Bool(false));
    }

    #[test]
    fn test_variable_use_node() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let v1 = Box::new(Node::new_variable_with_id("x".to_string(), 0));

        let mut add = Box::new(Node::new_add());
        add.add_child(v1);
        add.add_child(c1);

        base.add_child(add);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(vec![Value::Null]),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        assert!(evaluator.const_visit(base).is_err());
    }

    #[test]
    fn test_nested_expression() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let c2 = Box::new(Node::new_constant(2.0));
        let x = Box::new(Node::new_variable_with_id("x".to_string(), 0));
        let y = Box::new(Node::new_variable_with_id("y".to_string(), 1));
        let z = Box::new(Node::new_variable_with_id("z".to_string(), 2));

        let mut assign_x = Box::new(Node::new_assign());
        assign_x.add_child(x.clone());
        assign_x.add_child(c1);

        let mut assign_y = Box::new(Node::new_assign());
        assign_y.add_child(y.clone());
        assign_y.add_child(c2);

        let mut add = Box::new(Node::new_add());
        add.add_child(x.clone());
        add.add_child(y.clone());

        let mut assign_z = Box::new(Node::new_assign());
        assign_z.add_child(z);
        assign_z.add_child(add);

        base.add_child(assign_x);
        base.add_child(assign_y);
        base.add_child(assign_z);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(vec![Value::Null, Value::Null, Value::Null]),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.variables().pop().unwrap(), Value::Number(3.0));
    }

    #[test]
    fn test_equal() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let c2 = Box::new(Node::new_constant(1.0));

        let mut equal = Box::new(Node::new_equal());
        equal.add_child(c1);
        equal.add_child(c2);

        base.add_child(equal);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.boolean_stack().pop().unwrap(), true);
    }

    #[test]
    fn test_superior() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(2.0));
        let c2 = Box::new(Node::new_constant(1.0));

        let mut and = Box::new(Node::new_superior());
        and.add_child(c1);
        and.add_child(c2);

        base.add_child(and);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.boolean_stack().pop().unwrap(), true);
    }

    #[test]
    fn test_inferior() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let c2 = Box::new(Node::new_constant(2.0));

        let mut and = Box::new(Node::new_inferior());
        and.add_child(c1);
        and.add_child(c2);

        base.add_child(and);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.boolean_stack().pop().unwrap(), true);
    }

    #[test]
    fn test_superior_or_equal() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(2.0));
        let c2 = Box::new(Node::new_constant(1.0));

        let mut and = Box::new(Node::new_superior_or_equal());
        and.add_child(c1);
        and.add_child(c2);

        base.add_child(and);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.boolean_stack().pop().unwrap(), true);
    }

    #[test]
    fn test_inferior_or_equal() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let c2 = Box::new(Node::new_constant(2.0));

        let mut and = Box::new(Node::new_inferior_or_equal());
        and.add_child(c1);
        and.add_child(c2);

        base.add_child(and);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.boolean_stack().pop().unwrap(), true);
    }

    #[test]
    fn test_and() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let c2 = Box::new(Node::new_constant(1.0));

        let mut equal_1 = Box::new(Node::new_equal());
        equal_1.add_child(c1.clone());
        equal_1.add_child(c2.clone());

        let mut equal_2 = Box::new(Node::new_equal());
        equal_2.add_child(c1.clone());
        equal_2.add_child(c2.clone());

        let mut and = Box::new(Node::new_and());
        and.add_child(equal_1.clone());
        and.add_child(equal_2.clone());

        base.add_child(equal_1.clone());
        base.add_child(equal_2.clone());
        base.add_child(and);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.boolean_stack().pop().unwrap(), true);
    }

    #[test]
    fn test_or() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let c2 = Box::new(Node::new_constant(1.0));

        let mut equal_1 = Box::new(Node::new_equal());
        equal_1.add_child(c1.clone());
        equal_1.add_child(c2.clone());

        let mut equal_2 = Box::new(Node::new_equal());
        equal_2.add_child(c1.clone());
        equal_2.add_child(c2.clone());

        let mut or = Box::new(Node::new_or());
        or.add_child(equal_1.clone());
        or.add_child(equal_2.clone());

        base.add_child(equal_1.clone());
        base.add_child(equal_2.clone());
        base.add_child(or);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.boolean_stack().pop().unwrap(), true);
    }

    #[test]
    fn test_not() {
        let mut base = Box::new(Node::new_base());

        let c1 = Box::new(Node::new_constant(1.0));
        let c2 = Box::new(Node::new_constant(1.0));

        let mut equal = Box::new(Node::new_equal());
        equal.add_child(c1.clone());
        equal.add_child(c2.clone());

        let mut not = Box::new(Node::new_not());
        not.add_child(equal.clone());

        base.add_child(equal.clone());
        base.add_child(not.clone());

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(Vec::new()),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };
        evaluator.const_visit(base).unwrap();
        assert_eq!(evaluator.boolean_stack().pop().unwrap(), false);
    }

    #[test]
    fn test_if() {
        let mut base = Box::new(Node::new_base());

        let x = Box::new(Node::new_variable_with_id("x".to_string(), 0));
        let c1 = Box::new(Node::new_constant(1.0));

        let mut assing_x = Box::new(Node::new_assign());
        assing_x.add_child(x.clone());
        assing_x.add_child(c1.clone());

        let mut if_node = Box::new(Node::new_if());

        let mut equal = Box::new(Node::new_equal());

        equal.add_child(x.clone());
        equal.add_child(c1.clone());

        if_node.add_child(equal.clone());

        let mut add = Box::new(Node::new_add());
        add.add_child(x.clone());
        add.add_child(c1.clone());
        let mut assing_x_2 = Box::new(Node::new_assign());
        assing_x_2.add_child(x);
        assing_x_2.add_child(add);

        if_node.add_child(assing_x_2.clone());

        base.add_child(assing_x);
        base.add_child(if_node);

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(vec![Value::Null]),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };

        evaluator.const_visit(base).unwrap();
        assert_eq!(evaluator.variables().pop().unwrap(), Value::Number(2.0));
    }

    #[test]
    fn test_if_new_variable() {
        let base = Box::new(Node::Base(vec![
            Box::new(Node::Assign(vec![
                Box::new(Node::Variable(Vec::new(), "x".to_string(), 0.into())),
                Box::new(Node::Constant(2.0)),
            ])),
            Box::new(Node::If(
                vec![
                    Box::new(Node::Equal(vec![
                        Box::new(Node::Variable(Vec::new(), "x".to_string(), 0.into())),
                        Box::new(Node::Constant(1.0)),
                    ])),
                    Box::new(Node::Assign(vec![
                        Box::new(Node::Variable(Vec::new(), "z".to_string(), 1.into())),
                        Box::new(Node::Constant(3.0)),
                    ])),
                    Box::new(Node::Assign(vec![
                        Box::new(Node::Variable(Vec::new(), "w".to_string(), 2.into())),
                        Box::new(Node::Constant(4.0)),
                    ])),
                ],
                None,
            )),
        ]));

        let evaluator = ExpressionEvaluator {
            variables: Mutex::new(vec![Value::Null, Value::Null, Value::Null]),
            digit_stack: Mutex::new(Vec::new()),
            boolean_stack: Mutex::new(Vec::new()),
            is_lhs_variable: Mutex::new(false),
            lhs_variable: Mutex::new(None),
            current_event: None,
        };
        evaluator.const_visit(base).unwrap();

        assert_eq!(evaluator.variables().get(0).unwrap(), &Value::Number(2.0));
        assert_eq!(evaluator.variables().get(1).unwrap(), &Value::Null);
        assert_eq!(evaluator.variables().get(2).unwrap(), &Value::Null);
    }
}

#[cfg(test)]
mod script_tests {
    use crate::{
        nodes::{
            expressionevaluator::Value,
            expressionindexer::ExpressionIndexer,
            traits::{NodeConstVisitor, NodeVisitor},
        },
        parsers::{lexer::Lexer, parser::Parser},
    };

    use super::ExpressionEvaluator;

    #[test]
    fn test_simple_adding_script() {
        let script = "
            x = 1;
            y = 2;
            z = x + y;
            "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        tokens.iter().for_each(|t| println!("{:?}", t));
        let nodes = Parser::new(tokens).parse().unwrap();

        let indexer = ExpressionIndexer::new();
        indexer.visit(&nodes);

        let evaluator = ExpressionEvaluator::new().with_variables(indexer.get_size());
        evaluator.const_visit(nodes).unwrap();

        assert_eq!(*evaluator.variables().get(0).unwrap(), Value::Number(1.0));
    }

    #[test]
    fn test_if_script() {
        let script = "
            x = 2;
            y = 2;
            z = x + y;
            if x == 1 then
                z = 3;
            end
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let indexer = ExpressionIndexer::new();
        indexer.visit(&nodes);

        let evaluator = ExpressionEvaluator::new().with_variables(indexer.get_size());
        evaluator.const_visit(nodes).unwrap();

        assert_eq!(*evaluator.variables().get(0).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(1).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(2).unwrap(), Value::Number(4.0));
    }

    #[test]
    fn test_nested_if_script() {
        let script = "
            x = 2;
            y = 2;
            z = x + y;
            if x == 1 then
                z = 3;
            else
                z = 4;
            end
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let indexer = ExpressionIndexer::new();
        indexer.visit(&nodes);

        let evaluator = ExpressionEvaluator::new().with_variables(indexer.get_size());
        evaluator.const_visit(nodes).unwrap();

        println!("{:?}", evaluator.variables());

        assert_eq!(*evaluator.variables().get(0).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(1).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(2).unwrap(), Value::Number(4.0));
    }

    #[test]
    fn test_nested_if_else_script() {
        let script = "
            x = 2;
            y = 2;
            z = x + y;
            if x == 1 then
                z = 3;
            else
                if y == 1 then
                    z = 4;
                else
                    z = 5;
                end
            end
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let indexer = ExpressionIndexer::new();
        indexer.visit(&nodes);

        let evaluator = ExpressionEvaluator::new().with_variables(indexer.get_size());
        evaluator.const_visit(nodes).unwrap();

        assert_eq!(*evaluator.variables().get(0).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(1).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(2).unwrap(), Value::Number(5.0));
    }

    #[test]
    fn test_new_variable_in_if_script() {
        let script = "
            x = 2;
            y = 2;
            z = x + y;
            if x == 1 then
                z = 3;
                w = 4;
            end
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let indexer = ExpressionIndexer::new();
        indexer.visit(&nodes);

        let evaluator = ExpressionEvaluator::new().with_variables(indexer.get_size());
        evaluator.const_visit(nodes).unwrap();

        assert_eq!(*evaluator.variables().get(0).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(1).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(2).unwrap(), Value::Number(4.0));
        assert_eq!(*evaluator.variables().get(3).unwrap(), Value::Null);

        let script = "
            x = 2;
            y = 2;
            z = x + y;
            if x == 2 then
                z = 3;
                w = 4;
            end
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let indexer = ExpressionIndexer::new();
        indexer.visit(&nodes);

        let evaluator = ExpressionEvaluator::new().with_variables(indexer.get_size());
        evaluator.const_visit(nodes).unwrap();

        assert_eq!(*evaluator.variables().get(0).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(1).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(2).unwrap(), Value::Number(3.0));
        assert_eq!(*evaluator.variables().get(3).unwrap(), Value::Number(4.0));
    }

    #[test]
    fn test_nested_if_else_script_2() {
        let script = "
            x = 2;
            y = 2;
            z = x + y;
            if x == 1 then
                z = 3;
            else
                if y == 1 then
                    z = 4;
                else
                    z = 5;
                end
            end
        "
        .to_string();

        let tokens = Lexer::new(script).tokenize().unwrap();
        let nodes = Parser::new(tokens).parse().unwrap();

        let indexer = ExpressionIndexer::new();
        indexer.visit(&nodes);

        let evaluator = ExpressionEvaluator::new().with_variables(indexer.get_size());
        evaluator.const_visit(nodes).unwrap();

        assert_eq!(*evaluator.variables().get(0).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(1).unwrap(), Value::Number(2.0));
        assert_eq!(*evaluator.variables().get(2).unwrap(), Value::Number(5.0));
    }
}
