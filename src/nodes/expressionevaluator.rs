use std::cell::RefCell;

use super::{
    node::Node,
    traits::{ConstVisitable, NodeConstVisitor},
};

//type MarketData = Vec<f64>;
#[allow(unused)]
pub struct ExpressionEvaluator {
    variables: RefCell<Vec<f64>>,
    digit_stack: RefCell<Vec<f64>>,
    boolean_stack: RefCell<Vec<bool>>,
    is_lhs_variable: RefCell<bool>,
    lhs_variable: RefCell<Option<Box<Node>>>,
    current_event: Option<usize>,
}

impl ExpressionEvaluator {
    pub fn new() -> Self {
        ExpressionEvaluator {
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        }
    }

    pub fn with_variables(&self, n: usize) {
        self.variables.replace(vec![0.0; n]);
    }

    pub fn variables(&self) -> Vec<f64> {
        self.variables.borrow().clone()
    }

    pub fn digit_stack(&self) -> Vec<f64> {
        self.digit_stack.borrow().clone()
    }

    pub fn boolean_stack(&self) -> Vec<bool> {
        self.boolean_stack.borrow().clone()
    }
}

impl NodeConstVisitor for ExpressionEvaluator {
    fn const_visit(&self, node: Box<Node>) {
        //println!("Visiting node: {:?}", node);
        match node.as_ref() {
            Node::Base(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));
            }
            Node::Variable(_, _, index) => {
                if *self.is_lhs_variable.borrow() {
                    self.lhs_variable.replace(Some(node));
                } else {
                    match index {
                        None => panic!("Variable index not found"),
                        Some(index) => {
                            let value = self.variables.borrow()[*index];
                            self.digit_stack.borrow_mut().push(value);
                        }
                    }
                }
            }
            Node::Constant(value) => {
                self.digit_stack.borrow_mut().push(*value);
            }
            Node::Add(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(left + right);
            }
            Node::Subtract(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(left - right);
            }
            Node::Multiply(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(left * right);
            }
            Node::Divide(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(left / right);
            }
            Node::Assign(children) => {
                *self.is_lhs_variable.borrow_mut() = true;
                children.get(0).unwrap().const_accept(self);

                *self.is_lhs_variable.borrow_mut() = false;
                children.get(1).unwrap().const_accept(self);

                let value = self.digit_stack.borrow_mut().pop().unwrap();
                let variable = self.lhs_variable.borrow_mut().take().unwrap();
                match variable.as_ref() {
                    Node::Variable(_, _, index) => match index {
                        None => panic!("Variable index not found"),
                        Some(index) => self.variables.borrow_mut()[*index] = value,
                    },
                    _ => panic!("Invalid variable node"),
                }
            }
            Node::NotEqual(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.boolean_stack
                    .borrow_mut()
                    .push((right - left).abs() > f64::EPSILON);
            }
            Node::And(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.boolean_stack.borrow_mut().pop().unwrap();
                let left = self.boolean_stack.borrow_mut().pop().unwrap();
                self.boolean_stack.borrow_mut().push(left && right);
            }
            Node::Or(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.boolean_stack.borrow_mut().pop().unwrap();
                let left = self.boolean_stack.borrow_mut().pop().unwrap();
                self.boolean_stack.borrow_mut().push(left || right);
            }
            Node::Not(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let value = self.boolean_stack.borrow_mut().pop().unwrap();
                self.boolean_stack.borrow_mut().push(!value);
            }
            Node::Superior(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.boolean_stack.borrow_mut().push(left > right);
            }
            Node::Inferior(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.boolean_stack.borrow_mut().push(left < right);
            }
            Node::SuperiorOrEqual(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.boolean_stack.borrow_mut().push(left >= right);
            }
            Node::InferiorOrEqual(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.boolean_stack.borrow_mut().push(left <= right);
            }
            Node::True => {
                self.boolean_stack.borrow_mut().push(true);
            }

            Node::False => {
                self.boolean_stack.borrow_mut().push(false);
            }
            Node::Equal(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.boolean_stack
                    .borrow_mut()
                    .push((right - left).abs() < f64::EPSILON);
            }
            Node::UnaryPlus(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));
            }
            Node::UnaryMinus(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let value = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(-value);
            }
            Node::Min(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(left.min(right));
            }
            Node::Max(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(left.max(right));
            }
            Node::Pow(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));

                let right = self.digit_stack.borrow_mut().pop().unwrap();
                let left = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(left.powf(right));
            }
            Node::Ln(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));
                let top = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(top.ln());
            }
            Node::Exp(children) => {
                children
                    .iter()
                    .for_each(|child| self.const_visit(child.clone()));
                let top = self.digit_stack.borrow_mut().pop().unwrap();
                self.digit_stack.borrow_mut().push(top.exp());
            }
            Node::If(children, first_else) => {
                // Evaluate the condition
                children.get(0).unwrap().const_accept(self);
                // Pop the condition result
                let is_true = self.boolean_stack.borrow_mut().pop().unwrap();

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
            }
        }
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
        evaluator.const_visit(base);

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
        evaluator.const_visit(base);

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
        evaluator.const_visit(base);

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
        evaluator.const_visit(base);

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
            variables: RefCell::new(vec![0.0]),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

        assert_eq!(evaluator.variables().pop().unwrap(), 1.0);
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
            variables: RefCell::new(vec![0.0]),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

        assert_eq!(evaluator.digit_stack().pop().unwrap(), 1.0);
    }

    #[test]
    fn test_nested_expression() {
        // x = 1
        // y = 2
        // z = x + y

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
            variables: RefCell::new(vec![0.0, 0.0, 0.0]),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

        assert_eq!(evaluator.variables().pop().unwrap(), 3.0);
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
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

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
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

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
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

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
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

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
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

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
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

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
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);

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
            variables: RefCell::new(Vec::new()),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };
        evaluator.const_visit(base);
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
            variables: RefCell::new(vec![0.0]),
            digit_stack: RefCell::new(Vec::new()),
            boolean_stack: RefCell::new(Vec::new()),
            is_lhs_variable: RefCell::new(false),
            lhs_variable: RefCell::new(None),
            current_event: None,
        };

        evaluator.const_visit(base);
        assert_eq!(evaluator.variables().pop().unwrap(), 2.0);
    }

    // #[test]
    // fn test_for() {
    //     let mut base = Box::new(Node::new_base());

    //     let x = Box::new(Node::new_variable_with_id("x".to_string(), 0));
    //     let c1 = Box::new(Node::new_constant(1.0));

    //     let mut assing_x = Box::new(Node::new_assign());
    //     assing_x.add_child(x.clone());
    //     assing_x.add_child(c1.clone());

    //     let mut for_node = Box::new(Node::new_for_with_iterations(1));
    //     for_node.add_child(assing_x.clone());
    //     for_node.add_child(c1.clone());

    //     let mut add = Box::new(Node::new_add());
    //     add.add_child(x.clone());
    //     add.add_child(c1.clone());
    //     let mut assing_x_2 = Box::new(Node::new_assign());
    //     assing_x_2.add_child(x);
    //     assing_x_2.add_child(add);

    //     for_node.add_child(assing_x_2.clone());

    //     base.add_child(for_node);

    //     let evaluator = ExpressionEvaluator {
    //         variables: RefCell::new(vec![0.0]),
    //         digit_stack: RefCell::new(Vec::new()),
    //         boolean_stack: RefCell::new(Vec::new()),
    //         is_lhs_variable: RefCell::new(false),
    //         lhs_variable: RefCell::new(None),
    //         current_event: None,
    //     };

    //     evaluator.const_visit(base);
    //     assert_eq!(evaluator.variables().pop().unwrap(), 2.0);
    // }
}
