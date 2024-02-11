use super::nodes::*;
use crate::lexer::{Token, TokenType};

pub struct Parser<'a> {
    tokens: &'a [Token<'a>],
    current_token: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token<'a>]) -> Self {
        Parser {
            tokens,
            current_token: 0,
        }
    }

    // Getter for current token
    pub fn current_token(&self) -> Option<&Token<'a>> {
        self.tokens.get(self.current_token)
    }

    pub fn current_token_index(&self) -> usize {
        self.current_token
    }

    pub fn peek(&self, offset: usize) -> Option<&Token<'a>> {
        self.tokens.get(self.current_token + offset)
    }

    // Advance the current token by one.
    pub fn advance(&mut self, offset: usize) -> () {
        self.current_token += offset;
    }

    pub fn parse(&mut self) -> NodeProgram<'a> {
        let mut statements = Vec::new();

        while self.peek(self.current_token).is_some() {
            match self.parse_statement() {
                Some(statement) => statements.push(statement),
                None => panic!(
                    "Expected statement starting with token {:?} but did not get one.",
                    self.peek(self.current_token)
                ),
            }
        }

        NodeProgram { statements }
    }
}
