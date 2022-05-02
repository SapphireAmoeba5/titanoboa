use std::str::ParseBoolError;

use crate::interpreter::token::Token;
use crate::interpreter::Type;

#[derive(Debug)]
pub struct ParseTree {
    pub tree: Vec<ParseTreeNode>,
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
