use super::traits::{ConstVisitable, NodeConstVisitor, NodeVisitor, Visitable};

pub type ExpressionTree = Box<Node>;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Base(Vec<ExpressionTree>),

    // variables
    Variable(Vec<ExpressionTree>, String, Option<usize>),
    Constant(f64),

    // math
    Add(Vec<ExpressionTree>),
    Subtract(Vec<ExpressionTree>),
    Multiply(Vec<ExpressionTree>),
    Divide(Vec<ExpressionTree>),
    Assign(Vec<ExpressionTree>),
    Min(Vec<ExpressionTree>),
    Max(Vec<ExpressionTree>),
    Exp(Vec<ExpressionTree>),
    Pow(Vec<ExpressionTree>),
    Ln(Vec<ExpressionTree>),

    // unary
    UnaryPlus(Vec<ExpressionTree>),
    UnaryMinus(Vec<ExpressionTree>),

    // logic
    True,
    False,
    Equal(Vec<ExpressionTree>),
    NotEqual(Vec<ExpressionTree>),
    And(Vec<ExpressionTree>),
    Or(Vec<ExpressionTree>),
    Not(Vec<ExpressionTree>),
    Superior(Vec<ExpressionTree>),
    Inferior(Vec<ExpressionTree>),
    SuperiorOrEqual(Vec<ExpressionTree>),
    InferiorOrEqual(Vec<ExpressionTree>),

    // control flow
    If(Vec<ExpressionTree>, Option<usize>),
}

impl Node {
    pub fn new_base() -> Node {
        Node::Base(Vec::new())
    }

    pub fn new_add() -> Node {
        Node::Add(Vec::new())
    }

    pub fn new_subtract() -> Node {
        Node::Subtract(Vec::new())
    }

    pub fn new_multiply() -> Node {
        Node::Multiply(Vec::new())
    }

    pub fn new_divide() -> Node {
        Node::Divide(Vec::new())
    }

    pub fn new_variable(name: String) -> Node {
        Node::Variable(Vec::new(), name, None)
    }

    pub fn new_variable_with_id(name: String, id: usize) -> Node {
        Node::Variable(Vec::new(), name, Some(id))
    }

    pub fn new_min() -> Node {
        Node::Min(Vec::new())
    }

    pub fn new_max() -> Node {
        Node::Max(Vec::new())
    }

    pub fn new_exp() -> Node {
        Node::Exp(Vec::new())
    }

    pub fn new_ln() -> Node {
        Node::Ln(Vec::new())
    }

    pub fn new_pow() -> Node {
        Node::Pow(Vec::new())
    }

    pub fn new_constant(value: f64) -> Node {
        Node::Constant(value)
    }

    pub fn new_assign() -> Node {
        Node::Assign(Vec::new())
    }

    pub fn new_and() -> Node {
        Node::And(Vec::new())
    }

    pub fn new_or() -> Node {
        Node::Or(Vec::new())
    }

    pub fn new_not() -> Node {
        Node::Not(Vec::new())
    }

    pub fn new_superior() -> Node {
        Node::Superior(Vec::new())
    }

    pub fn new_inferior() -> Node {
        Node::Inferior(Vec::new())
    }

    pub fn new_superior_or_equal() -> Node {
        Node::SuperiorOrEqual(Vec::new())
    }

    pub fn new_equal() -> Node {
        Node::Equal(Vec::new())
    }

    pub fn new_if() -> Node {
        Node::If(Vec::new(), None)
    }

    pub fn new_unary_plus() -> Node {
        Node::UnaryPlus(Vec::new())
    }

    pub fn new_unary_minus() -> Node {
        Node::UnaryMinus(Vec::new())
    }

    pub fn new_inferior_or_equal() -> Node {
        Node::InferiorOrEqual(Vec::new())
    }

    pub fn new_not_equal() -> Node {
        Node::NotEqual(Vec::new())
    }

    pub fn new_true() -> Node {
        Node::True
    }

    pub fn new_false() -> Node {
        Node::False
    }

    pub fn add_child(&mut self, child: ExpressionTree) {
        match self {
            Node::Base(children) => children.push(child),
            Node::Add(children) => children.push(child),
            Node::Subtract(children) => children.push(child),
            Node::Multiply(children) => children.push(child),
            Node::Divide(children) => children.push(child),
            Node::Variable(children, _, _) => children.push(child),
            Node::Assign(children) => children.push(child),
            Node::And(children) => children.push(child),
            Node::Or(children) => children.push(child),
            Node::Not(children) => children.push(child),
            Node::Superior(children) => children.push(child),
            Node::Inferior(children) => children.push(child),
            Node::SuperiorOrEqual(children) => children.push(child),
            Node::InferiorOrEqual(children) => children.push(child),
            Node::Equal(children) => children.push(child),
            Node::If(children, _) => children.push(child),
            Node::UnaryPlus(children) => children.push(child),
            Node::UnaryMinus(children) => children.push(child),
            Node::Min(children) => children.push(child),
            Node::Max(children) => children.push(child),
            Node::Exp(children) => children.push(child),
            Node::Ln(children) => children.push(child),
            Node::Pow(children) => children.push(child),
            Node::NotEqual(children) => children.push(child),
            Node::True => panic!("Cannot add child to true node"),
            Node::False => panic!("Cannot add child to false node"),
            Node::Constant(_) => panic!("Cannot add child to constant node"),
        }
    }

    pub fn children(&self) -> &Vec<ExpressionTree> {
        match self {
            Node::Base(children) => children,
            Node::Add(children) => children,
            Node::Subtract(children) => children,
            Node::Multiply(children) => children,
            Node::Divide(children) => children,
            Node::Variable(children, _, _) => children,
            Node::Assign(children) => children,
            Node::And(children) => children,
            Node::Or(children) => children,
            Node::Not(children) => children,
            Node::Superior(children) => children,
            Node::Inferior(children) => children,
            Node::SuperiorOrEqual(children) => children,
            Node::InferiorOrEqual(children) => children,
            Node::Equal(children) => children,
            Node::If(children, _) => children,
            Node::UnaryPlus(children) => children,
            Node::UnaryMinus(children) => children,
            Node::Min(children) => children,
            Node::Max(children) => children,
            Node::Exp(children) => children,
            Node::Ln(children) => children,
            Node::Pow(children) => children,
            Node::NotEqual(children) => children,
            Node::True => panic!("Cannot get children from true node"),
            Node::False => panic!("Cannot get children from false node"),
            Node::Constant(_) => panic!("Cannot get children from constant node"),
        }
    }
}

impl Visitable for Box<Node> {
    fn accept(&mut self, visitor: &dyn NodeVisitor) {
        visitor.visit(self.clone());
    }
}

impl ConstVisitable for Box<Node> {
    fn const_accept(&self, visitor: &dyn NodeConstVisitor) {
        visitor.const_visit(self.clone());
    }
}
