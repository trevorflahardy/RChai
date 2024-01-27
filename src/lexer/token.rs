use super::constants::*;

#[derive(Debug)]
pub enum TokenType<'a> {
    Identifier { name: &'a str },
    IntegerLiteral { value: &'a str },
    Equals,
    Semi,
    LParen,
    RParen,
    Let,
}

#[derive(Debug)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub start: usize,
    pub end: usize,
}

pub fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == UNDERSCORE
}
