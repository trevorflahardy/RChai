use super::{nodes::NodeStatement, parser::Parser};
use crate::lexer::TokenType;

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> Option<NodeStatement<'a>> {
        let token = self.current_token()?;

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
}
