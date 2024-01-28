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

    fn peek(&self, offset: usize) -> Option<&Token<'a>> {
        self.tokens.get(self.current_token + offset)
    }

    // Advance the current token by one.
    fn advance(&mut self, offset: usize) -> () {
        self.current_token += offset;
    }

    fn parse_statement(&mut self) -> Option<NodeStatement<'a>> {
        let token = self.peek(self.current_token)?;

        match token.token_type {
            TokenType::Let => {
                // If we have a let statement, it must be followed by an identifier, an equal sign, and then
                // some sort of expression (int_lit, etc):
                todo!();
            }
            TokenType::Identifier { name } => {
                // We could have a function call, or a variable assignment.
                todo!();
            }
            TokenType::OpenBrace => {
                // We have a block statement.
                todo!();
            }
            _ => None,
        }
    }

    pub fn parse(&mut self) -> NodeModule<'a> {
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

        NodeModule { statements }
    }
}
