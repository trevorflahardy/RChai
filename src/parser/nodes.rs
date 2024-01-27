use crate::lexer::Token;
use std::boxed::Box;

pub trait Node {}
pub trait Expression: Node {}
pub trait Statement: Node {}

pub struct Module {
    pub body: Vec<Box<dyn Node>>,
}

pub struct Identifier<'a> {
    pub name: &'a str,
    pub token: &'a Token<'a>,
}

impl Node for Identifier<'_> {}

pub struct Assignment<'a> {
    pub targets: Vec<&'a Identifier<'a>>,
    pub value: &'a dyn Expression,
}

impl Node for Assignment<'_> {}

// Literals (int, string, and bool)
pub struct Constant<'a> {
    pub value: u32, // Will become dynamic later
    pub token: &'a Token<'a>,
}

impl Node for Constant<'_> {}
impl Expression for Constant<'_> {}
