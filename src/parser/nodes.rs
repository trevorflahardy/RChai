use crate::lexer::Token;

pub trait Node {}
pub trait NodeExpression: Node {}

pub struct NodeStatement<'a> {
    pub expression: &'a dyn NodeExpression,
    pub token: &'a Token<'a>,
}

impl Node for NodeStatement<'_> {}

pub struct NodeProgram<'a> {
    pub statements: Vec<NodeStatement<'a>>,
}

pub struct NodeIdentifier<'a> {
    pub name: &'a str,
    pub token: &'a Token<'a>,
}

impl Node for NodeIdentifier<'_> {}

pub struct NodeAssignment<'a> {
    pub identifier: &'a NodeIdentifier<'a>, // What is being assigned to.
    pub expression: &'a dyn NodeExpression, // The expression being assigned.
}

impl Node for NodeAssignment<'_> {}

// Literals (int, string, and bool)
pub struct NodeConstant<'a> {
    pub value: u32, // Will become dynamic later but only int supported
    pub token: &'a Token<'a>,
}

impl Node for NodeConstant<'_> {}
impl NodeExpression for NodeConstant<'_> {}
