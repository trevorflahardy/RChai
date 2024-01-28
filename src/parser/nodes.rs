use crate::lexer::Token;
use std::boxed::Box;

pub trait Node {}
pub trait NodeExpression: Node {}
pub trait NodeStatement: Node {}

pub struct NodeModule {
    pub body: Vec<Box<dyn Node>>,
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
