use std::{collections::HashMap, sync::Mutex};

use rustatlas::prelude::*;

use super::{node::Node, traits::NodeVisitor};

pub struct ExpressionIndexer {
    variables: Mutex<HashMap<String, usize>>,
    market_requests: Mutex<Vec<MarketRequest>>,
    numerarie_requests: Mutex<Vec<NumerarieRequest>>,
    reference_date: Option<Date>,
    local_currency: Option<Currency>,
}

impl NodeVisitor for ExpressionIndexer {
    type Output = Result<()>;
    fn visit(&self, node: &Box<Node>) -> Self::Output {
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
                children.iter().try_for_each(|child| self.visit(child))?;
                Ok(())
            }

            Node::Variable(children, name, opt_idx) => {
                children.iter().try_for_each(|child| self.visit(child))?;
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
                Ok(())
            }
            Node::Spot(currency, opt_idx) => {
                match opt_idx.get() {
                    Some(_) => {}
                    None => {
                        let size = self.market_requests.lock().unwrap().len();
                        let exchange_request = ExchangeRateRequest::new(
                            *currency,
                            self.local_currency,
                            self.reference_date,
                        );
                        let request = MarketRequest::new(size, None, None, Some(exchange_request));
                        self.market_requests.lock().unwrap().push(request.clone());
                        opt_idx.set(size).unwrap();
                    }
                };
                Ok(())
            }
            Node::Pays(_, opt_idx) => match opt_idx.get() {
                Some(_) => Ok(()),
                None => {
                    let size = self.numerarie_requests.lock().unwrap().len();
                    let request = NumerarieRequest::new(
                        self.reference_date
                            .ok_or(AtlasError::ValueNotSetErr("Reference date".to_string()))?,
                    );
                    self.numerarie_requests
                        .lock()
                        .unwrap()
                        .push(request.clone());
                    opt_idx.set(size).unwrap();
                    Ok(())
                }
            },
            _ => Ok(()),
        }
    }
}

impl ExpressionIndexer {
    pub fn new() -> ExpressionIndexer {
        ExpressionIndexer {
            variables: Mutex::new(HashMap::new()),
            market_requests: Mutex::new(Vec::new()),
            numerarie_requests: Mutex::new(Vec::new()),
            reference_date: None,
            local_currency: None,
        }
    }

    pub fn with_reference_date(mut self, date: Date) -> Self {
        self.reference_date = Some(date);
        self
    }

    pub fn with_local_currency(mut self, currency: Currency) -> Self {
        self.local_currency = Some(currency);
        self
    }

    pub fn get_variable_index(&self, variable_name: &str) -> Option<usize> {
        self.variables.lock().unwrap().get(variable_name).cloned()
    }

    pub fn get_variable_name(&self, variable_index: usize) -> Option<String> {
        self.variables
            .lock()
            .unwrap()
            .iter()
            .find(|(_, &v)| v == variable_index)
            .map(|(k, _)| k.clone())
    }

    pub fn get_variables(&self) -> Vec<String> {
        self.variables.lock().unwrap().keys().cloned().collect()
    }

    pub fn get_variable_indexes(&self) -> HashMap<String, usize> {
        self.variables.lock().unwrap().clone()
    }

    pub fn get_variables_size(&self) -> usize {
        self.variables.lock().unwrap().len()
    }

    pub fn get_market_requests(&self) -> Vec<MarketRequest> {
        self.market_requests.lock().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use super::*;
    use crate::nodes::node::Node;

    #[test]
    fn test_expression_indexer() {
        let indexer = ExpressionIndexer::new();
        let node = Box::new(Node::new_variable("x".to_string()));
        indexer.visit(&node).unwrap();
        let variables = indexer.variables.lock().unwrap();
        assert_eq!(variables.get("x"), Some(&0));
        print!("{:?}", node);
    }

    #[test]
    fn test_expression_indexer_multiple() {
        let indexer = ExpressionIndexer::new();
        let node = Box::new(Node::new_variable("x".to_string()));
        indexer.visit(&node).unwrap();
        let node = Box::new(Node::new_variable("y".to_string()));
        indexer.visit(&node).unwrap();
        let variables = indexer.variables.lock().unwrap();
        assert_eq!(variables.get("x"), Some(&0));
        assert_eq!(variables.get("y"), Some(&1));
    }

    #[test]
    fn test_spot_indexer() {
        let indexer = ExpressionIndexer::new();
        let node = Box::new(Node::Spot(Currency::USD, OnceLock::new()));
        indexer.visit(&node).unwrap();
        let market_requests = indexer.market_requests.lock().unwrap();
        assert_eq!(market_requests.len(), 1);

        let request = market_requests.get(0).unwrap();
        assert_eq!(request.id(), 0);
        assert_eq!(request.fx().unwrap().first_currency(), Currency::USD);
        assert_eq!(request.fx().unwrap().second_currency(), None);
        assert_eq!(request.fx().unwrap().reference_date(), None);
    }

    #[test]
    fn test_spot_indexer_multiple() {
        let indexer = ExpressionIndexer::new();
        let node = Box::new(Node::Spot(Currency::USD, OnceLock::new()));
        indexer.visit(&node).unwrap();
        let node = Box::new(Node::Spot(Currency::EUR, OnceLock::new()));
        indexer.visit(&node).unwrap();
        let market_requests = indexer.market_requests.lock().unwrap();
        assert_eq!(market_requests.len(), 2);

        let request = market_requests.get(0).unwrap();
        assert_eq!(request.id(), 0);
        assert_eq!(request.fx().unwrap().first_currency(), Currency::USD);
        assert_eq!(request.fx().unwrap().second_currency(), None);
        assert_eq!(request.fx().unwrap().reference_date(), None);

        let request = market_requests.get(1).unwrap();
        assert_eq!(request.id(), 1);
        assert_eq!(request.fx().unwrap().first_currency(), Currency::EUR);
        assert_eq!(request.fx().unwrap().second_currency(), None);
        assert_eq!(request.fx().unwrap().reference_date(), None);
    }

    #[test]
    fn test_pays_indexer() {
        let indexer = ExpressionIndexer::new().with_reference_date(Date::empty());
        let node = Box::new(Node::new_pays());
        indexer.visit(&node).unwrap();
        let numerarie_requests = indexer.numerarie_requests.lock().unwrap();
        assert_eq!(numerarie_requests.len(), 1);

        let request = numerarie_requests.get(0).unwrap();

        assert_eq!(request.reference_date(), Date::empty());
    }
}
