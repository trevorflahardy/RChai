// File: The lexer for the language. Parses the source code stream into a list of tokens.

// Constants that represent the tokens in the language.
const EQUALS: &'static str = "=";
const LET: &'static str = "let";
const SEMI: &'static str = ";";
const LPAREN: &'static str = "(";
const RPAREN: &'static str = ")";
const WHITESPACE: char = ' ';

pub enum TokenType {
    Identifier,
    IntegerLiteral,
    Equals,
    Let,
}

/// Represents an actual token from the source code. Contains a token type and a value.
pub struct Token<'a> {
    pub token_type: TokenType,
    pub value: &'a String,
}

// The Lexer for the language.
pub struct Lexer<'a> {
    source: &'a String,
    current_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a String) -> Lexer {
        Lexer {
            source: source,
            current_position: 0,
        }
    }

    /// Peeks into the current position of the source code stream and returns the character at that
    /// position. Optionally, can step forward by a certain amount.
    ///
    /// Will return None if the current position is out of bounds.
    fn peek(&self, offset: usize) -> Option<char> {
        let peek_position = self.current_position + offset;

        if peek_position >= self.source.len() {
            return None;
        }

        Some(self.source.chars().nth(peek_position).unwrap())
    }
    /// Consumes N characters from the source code stream and returns them as an Vec of chars. Will increrment the current position by N.
    /// Will return None if the current position is out of bounds.
    fn consume(&mut self, n: usize) -> Option<&str> {
        let start_position = self.current_position;
        let end_position = start_position + n;

        if end_position > self.source.len() {
            return None;
        }

        self.current_position = end_position;
        Some(&self.source[start_position..end_position])
    }

    /// The actual lexer function. Will parse the source code stream into a list of tokens.
    /// Returns a vector of tokens.
    pub fn parse(&mut self) -> Vec<Token> {
        let tokens: Vec<Token> = Vec::new();

        let mut buf: Vec<&str> = Vec::new();
        // While we haven't reached the end of the source code stream (aka peek is not None), continue to parse tokens.
        while self.peek(0).is_some() {
            let current_value = self.peek(0).unwrap();
            if current_value.is_alphabetic() {
                todo!();
            } else if current_value.is_digit(10) {
                // This is probably an integer literal
                todo!();
            } else if current_value == WHITESPACE {
                todo!();
            } else {
                // This is probably an identifier
                todo!();
            }
        }

        tokens
    }
}
