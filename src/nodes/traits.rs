use super::node::Node;

pub trait NodeVisitor {
    fn visit(&self, node: Box<Node>);
}

pub trait NodeConstVisitor {
    fn const_visit(&self, node: Box<Node>);
}

pub trait Visitable {
    fn accept(&mut self, visitor: &dyn NodeVisitor);
}

pub trait ConstVisitable {
    fn const_accept(&self, visitor: &dyn NodeConstVisitor);
}
