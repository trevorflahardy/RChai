use super::{
    constants::*,
    source::Source,
    token::{is_identifier_start, Token, TokenType},
};

// Macro for single constant tokens.
macro_rules! single_token {
    ($self:ident, $token_type:ident) => {{
        let start = $self.current_position;
        $self.current_position += 1;
        let end = $self.current_position;

        Some(Token {
            token_type: TokenType::$token_type,
            start,
            end,
        })
    }};
}

macro_rules! reserved_ident {
    ($self:ident, $token_type:ident, $start:ident, $end:ident) => {{
        Some(Token {
            token_type: TokenType::$token_type,
            $start,
            $end,
        })
    }};
}

// The Lexer for the language.
pub struct Lexer<'a> {
    source: &'a Source<'a>,
    current_position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Source) -> Self {
        Lexer {
            source,
            current_position: 0,
        }
    }

    pub fn iter(&'a self) -> Lexer<'a> {
        Lexer {
            source: self.source,
            current_position: 0,
        }
    }

    fn advance(&mut self, offset: usize) -> () {
        self.current_position += offset;
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Peek until the next non-whitespace character.
        match self
            .source
            .peek_until(self.current_position, |c| !c.is_whitespace())
        {
            None => return None,
            Some(position) => {
                self.current_position = position;
            }
        }

        // Match it for the next token.
        match self.source.peek(self.current_position) {
            None => None,
            Some(c) => {
                if is_identifier_start(c) {
                    let start = self.current_position;
                    self.advance(1);

                    // Peek until we find a non-alphabetic or identifier character.
                    match self
                        .source
                        .peek_until(self.current_position, |c| !is_identifier_start(c))
                    {
                        None => return None,
                        Some(end) => self.current_position = end,
                    }

                    let end = self.current_position;

                    // Check if the identifier is a specific keyword
                    let identifier = &self.source[start..end];
                    match identifier {
                        LET => reserved_ident!(self, Let, start, end),
                        NOT => reserved_ident!(self, Not, start, end),
                        OR => reserved_ident!(self, Or, start, end),
                        AND => reserved_ident!(self, And, start, end),
                        _ => Some(Token {
                            token_type: TokenType::Identifier { name: identifier },
                            start,
                            end,
                        }),
                    }
                } else if c.is_numeric() {
                    let start = self.current_position;
                    self.advance(1);

                    // Peek until we find a non-numeric character.
                    match self
                        .source
                        .peek_until(self.current_position, |c| !c.is_numeric())
                    {
                        None => return None,
                        Some(end) => self.current_position = end,
                    }

                    let end = self.current_position;

                    // Parse the integer
                    let integer = &self.source[start..self.current_position];
                    Some(Token {
                        token_type: TokenType::IntegerLiteral {
                            value: integer.parse().unwrap(),
                        },
                        start,
                        end,
                    })
                } else {
                    // This is some other type of character, check if it's some type of multi-character
                    // token (like != or ==)
                    match c {
                        EQUALS => match self.source.peek(self.current_position + 1) {
                            // If the next character is an equals sign, we have a double equals token
                            Some(next_char) if next_char == EQUALS => {
                                let start = self.current_position;
                                self.advance(2);
                                let end = self.current_position;

                                Some(Token {
                                    token_type: TokenType::EqualsEquals,
                                    start,
                                    end,
                                })
                            }
                            _ => single_token!(self, Equals),
                        },
                        SEMI => single_token!(self, Semi),
                        LPAREN => single_token!(self, LParen),
                        RPAREN => single_token!(self, RParen),
                        OPEN_BRACE => single_token!(self, OpenBrace),
                        CLOSE_BRACE => single_token!(self, CloseBrace),
                        PLUS => single_token!(self, Plus),
                        MINUS => single_token!(self, Minus),
                        ASTERISK => single_token!(self, Asterisk),
                        SLASH => single_token!(self, Slash),
                        PERCENT => single_token!(self, Percent),
                        LESS_THAN => single_token!(self, LessThan),
                        GREATER_THAN => single_token!(self, GreaterThan),
                        BITWISE_OR => single_token!(self, BitwiseOr),
                        BITWISE_AND => single_token!(self, BitwiseAnd),
                        BITWISE_XOR => single_token!(self, BitwiseXor),
                        BITWISE_NOT => single_token!(self, BitwiseNot),

                        // Check for != token
                        '!' => match self.source.peek(self.current_position + 1) {
                            // If the next character is an equals sign, we have a not equals token
                            Some(next_char) if next_char == EQUALS => {
                                let start = self.current_position;
                                self.advance(2);
                                let end = self.current_position;

                                Some(Token {
                                    token_type: TokenType::NotEquals,
                                    start,
                                    end,
                                })
                            }
                            _ => panic!("Unexpected character: {:?}", c),
                        },
                        _ => {
                            panic!("Unexpected character: {:?}", c);
                        }
                    }
                }
            }
        }
    }
}
