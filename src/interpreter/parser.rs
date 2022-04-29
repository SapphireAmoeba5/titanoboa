mod comments;
mod tokenbuilder;
mod tokenstream;

use crate::interpreter::token::Token;
use comments::strip_comments;
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
