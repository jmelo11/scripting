use std::{collections::HashMap, sync::Mutex};

use super::{node::Node, traits::NodeVisitor};

pub struct ExpressionIndexer {
    pub variables: Mutex<HashMap<String, usize>>,
}

impl NodeVisitor for ExpressionIndexer {
    type Output = ();
    fn visit(&self, node: &Box<Node>) {
        match node.as_ref() {
            Node::Base(children)
            | Node::Add(children)
            | Node::Subtract(children)
            | Node::Multiply(children)
            | Node::Divide(children)
            | Node::Assign(children)
            | Node::Min(children)
            | Node::Max(children)
            | Node::Exp(children)
            | Node::Pow(children)
            | Node::Ln(children)
            | Node::UnaryPlus(children)
            | Node::UnaryMinus(children)
            | Node::Equal(children)
            | Node::NotEqual(children)
            | Node::And(children)
            | Node::Or(children)
            | Node::Not(children)
            | Node::Superior(children)
            | Node::Inferior(children)
            | Node::SuperiorOrEqual(children)
            | Node::InferiorOrEqual(children)
            | Node::If(children, _) => {
                children.iter().for_each(|child| self.visit(child));
            }

            Node::Variable(children, name, opt_idx) => {
                children.iter().for_each(|child| self.visit(child));
                match opt_idx.get() {
                    Some(id) => {
                        self.variables.lock().unwrap().insert(name.clone(), *id);
                    }
                    None => {
                        // check if the variable is already in the hashmap
                        if self.variables.lock().unwrap().contains_key(name) {
                            let size = self.variables.lock().unwrap().get(name).unwrap().clone();
                            // Update the id of the variable
                            opt_idx.set(size).unwrap();
                        } else {
                            let size = self.variables.lock().unwrap().len();
                            self.variables.lock().unwrap().insert(name.clone(), size);
                            // Update the id of the variable
                            opt_idx.set(size).unwrap();
                        }
                    }
                };
            }

            _ => {}
        }
    }
}

impl ExpressionIndexer {
    pub fn new() -> ExpressionIndexer {
        ExpressionIndexer {
            variables: Mutex::new(HashMap::new()),
        }
    }

    pub fn get_index(&self, name: &str) -> Option<usize> {
        self.variables.lock().unwrap().get(name).cloned()
    }

    pub fn get_name(&self, index: usize) -> Option<String> {
        self.variables
            .lock()
            .unwrap()
            .iter()
            .find(|(_, &v)| v == index)
            .map(|(k, _)| k.clone())
    }

    pub fn get_variables(&self) -> Vec<String> {
        self.variables.lock().unwrap().keys().cloned().collect()
    }

    pub fn get_indexes(&self) -> HashMap<String, usize> {
        self.variables.lock().unwrap().clone()
    }

    pub fn get_size(&self) -> usize {
        self.variables.lock().unwrap().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::nodes::node::Node;

    #[test]
    fn test_expression_indexer() {
        let indexer = ExpressionIndexer::new();
        let node = Box::new(Node::new_variable("x".to_string()));
        indexer.visit(&node);
        let variables = indexer.variables.lock().unwrap();
        assert_eq!(variables.get("x"), Some(&0));
        print!("{:?}", node);
    }

    #[test]
    fn test_expression_indexer_multiple() {
        let indexer = ExpressionIndexer::new();
        let node = Box::new(Node::new_variable("x".to_string()));
        indexer.visit(&node);
        let node = Box::new(Node::new_variable("y".to_string()));
        indexer.visit(&node);
        let variables = indexer.variables.lock().unwrap();
        assert_eq!(variables.get("x"), Some(&0));
        assert_eq!(variables.get("y"), Some(&1));
    }
}
