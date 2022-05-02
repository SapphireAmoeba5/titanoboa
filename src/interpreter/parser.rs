mod comments;
mod parsetree;
mod tokenbuilder;
mod tokenstream;

use std::ops::Deref;

use crate::interpreter::token::Token;
use crate::interpreter::Type;
use comments::strip_comments;
use parsetree::*;
use tokenbuilder::TokenBuilder;
use tokenstream::TokenStream;
pub struct Parser {}

impl Parser {
    pub fn run(source: String) -> TokenStream {
        let source = strip_comments(&source);
        let mut builder = TokenBuilder::new();

        for ch in source.chars() {
            builder.push_char(ch);
        }

        builder.finalize()
    }
}
