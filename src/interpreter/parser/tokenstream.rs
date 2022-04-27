use std::ops;
use crate::interpreter::token::Token;

#[derive(Debug, Clone)]
pub struct TokenStream {
    tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
        }
    }

    pub fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    
    pub fn raw(&self) -> &[Token] {
        &self.tokens[..]
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