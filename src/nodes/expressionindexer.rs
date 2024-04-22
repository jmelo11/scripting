use std::collections::HashMap;

use super::{node::Node, traits::NodeVisitor};

pub struct ExpressionIndexer {
    pub variables: HashMap<String, usize>,
}

impl NodeVisitor for ExpressionIndexer {
    fn visit(&self, node: Box<Node>) {}
}
