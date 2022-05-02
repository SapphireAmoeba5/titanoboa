use crate::interpreter::parser::TokenStream;
use std::ops;
use std::str::ParseBoolError;

use crate::interpreter::token::Token;
use crate::interpreter::Type;

pub fn parse_token_stream(tokens: Vec<TokenStream>) -> ParseTree {
    let mut parse_tree = ParseTree::new();

    parse_tree
}

#[derive(Debug)]
pub struct ParseTree {
    pub tree: Vec<ParseTreeNode>,
}

impl ParseTree {
    pub fn new() -> Self {
        Self { tree: Vec::new() }
    }

    pub fn push(&mut self, node: ParseTreeNode) {
        self.tree.push(node);
    }
}

impl ops::Deref for ParseTree {
    type Target = [ParseTreeNode];

    fn deref(&self) -> &Self::Target {
        &self.tree[..]
    }
}

#[derive(Debug)]
pub enum ParseTreeNode {
    ExpressionNode(Expression),
    IfStatementNode {
        line: usize,
        start: usize,
        end: usize,
        test: Expression,
        consequent: Vec<ParseBoolError>,
        alternate: Vec<ParseBoolError>,
    },
}

#[derive(Debug)]
pub enum Expression {
    Identifier {
        line: usize,
        start: usize,
        end: usize,
        name: String,
    },
    Literal {
        line: usize,
        start: usize,
        end: usize,
        value: Type,
    },
    BinaryExpression {
        line: usize,
        start: usize,
        end: usize,
        left: Box<Expression>,
        operator: String,
        right: Box<Expression>,
    },
}
