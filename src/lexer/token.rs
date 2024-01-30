use super::constants::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType<'a> {
    Identifier { name: &'a str },
    IntegerLiteral { value: u32 },

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
    NotEquals,
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

/// Represents a token passed along from the lexer through its iterator.
// # Type Parameters
// - `'a`: The lifetime of the source's contents.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Token<'a> {
    pub token_type: TokenType<'a>,
    pub start: usize,
    pub end: usize,
}

/// Checks if the given character is a valid identifier start.
/// # Parameters
/// - `c`: The character to check.
pub fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == UNDERSCORE
}
