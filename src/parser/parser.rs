use super::nodes::*;
use crate::lexer::Lexer;

pub struct Parser<'a> {
    lexer: &'a Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Self {
        Parser { lexer }
    }

    pub fn parse(&mut self) -> Module {
        let mut body = Vec::new();

        // Collect all lexer tokens into a vector
        let lexer_tokens: Vec<_> = self.lexer.iter().collect();

        Module { body }
    }
}
