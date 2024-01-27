use super::constants::*;

#[derive(Debug)]
pub enum TokenType<'a> {
    Identifier { name: &'a str },
    IntegerLiteral { value: &'a str },

    // Assignment operations
    Equals,
    Let,

    // Statement terminators
    Semi,

    // Calling conventions
    LParen,
    RParen,

    // Scope control
    OpenBrace,
    CloseBrace,

    // Arithmetic operations
    Plus,
    Minus,
    Asterisk,
    Slash,
    Percent,

    // Comparison operations
    EqualsEquals,
    LessThan,
    GreaterThan,
    Not,
    Or,
    And,

    // Bitwise operations
    BitwiseOr,
    BitwiseAnd,
    BitwiseXor,
    BitwiseNot,
    BitwiseLeftShift,
    BitwiseRightShift,
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
