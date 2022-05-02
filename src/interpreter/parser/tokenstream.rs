mod try_parse;

use crate::interpreter::token::Token;
use crate::interpreter::types::Type;
use std::ops;
use try_parse::try_parse_number;

#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    /// Push a token as a string and automatically determine the kind of token it is
    pub fn push_str(&mut self, token: &str, start: usize, end: usize, line: usize) {
        // Return early if the token is only whitespace excluding a newline
        if token != "\n" && token.trim().is_empty() {
            return;
        }
        let token = self.determine_token(token, start, end, line);
        self.tokens.push(token);
    }
}

impl ops::Deref for TokenStream {
    type Target = [Token];

    fn deref(&self) -> &Self::Target {
        &self.tokens[..]
    }
}

impl TokenStream {
    fn determine_token(&self, token: &str, start: usize, end: usize, line: usize) -> Token {
        match token {
            "fn" => Token::Fn { line, start, end },
            "->" => Token::Arrow { line, start, end },
            "," => Token::Comma { line, start, end },
            "." => Token::Period { line, start, end },
            ":" => Token::Colon { line, start, end },
            "=" => Token::Assignment { line, start, end },
            "==" => Token::Equality { line, start, end },
            "!=" => Token::EqualityNot { line, start, end },
            ">" => Token::GreaterThan { line, start, end },
            ">=" => Token::GreaterThanEqual { line, start, end },
            "<" => Token::LessThan { line, start, end },
            "<=" => Token::LessThanEqual { line, start, end },
            "return" => Token::Return { line, start, end },
            "int" => Token::Int { line, start, end },
            "uint" => Token::Uint { line, start, end },
            "float" => Token::Float { line, start, end },
            "bool" => Token::Bool { line, start, end },
            "char" => Token::Char { line, start, end },
            "string" => Token::String { line, start, end },
            "None" => Token::None { line, start, end },
            "true" => Token::True { line, start, end },
            "false" => Token::False { line, start, end },
            "begin" => Token::Begin { line, start, end },
            "then" => Token::Then { line, start, end },
            "do" => Token::Do { line, start, end },
            "end" => Token::End { line, start, end },
            "(" => Token::OpeningBracket { line, start, end },
            ")" => Token::ClosingBracket { line, start, end },
            "\n" => Token::Newline { line, start, end },
            // Parse the token type
            _ => self.parse_token(token, start, end, line),
        }
    }

    fn parse_token(&self, token: &str, start: usize, end: usize, line: usize) -> Token {
        match try_parse_number(token) {
            Some(Type::Int(int)) => {
                return Token::IntegerLiteral {
                    line,
                    start,
                    end,
                    value: int,
                }
            }
            Some(Type::Uint(uint)) => {
                return Token::UIntegerLiteral {
                    line,
                    start,
                    end,
                    value: uint,
                };
            }
            Some(Type::Float(float)) => {
                return Token::FloatLiteral {
                    line,
                    start,
                    end,
                    value: float,
                };
            }
            None => {}
            _ => {
                unreachable!()
            }
        }

        // An identifier cannot begin with any kind of numeric ascii character
        if !(token.as_bytes()[0] as char).is_numeric() {
            return Token::Identifier {
                line,
                start,
                end,
                value: token.to_string(),
            };
        }

        Token::Unknown {
            line,
            start,
            end,
            token: token.to_string(),
        }
    }
}

impl ops::Index<usize> for TokenStream {
    type Output = Token;
    fn index<'a>(&'a self, index: usize) -> &'a Token {
        &self.tokens[index]
    }
}

impl ops::IndexMut<usize> for TokenStream {
    fn index_mut(&mut self, index: usize) -> &mut Token {
        &mut self.tokens[index]
    }
}
