pub mod parser;

use crate::lexer::tokens::*;

#[derive(Debug, PartialEq)]
pub enum Ast {
    EOF,
    ValueNode(LiteralKind),
    IdentifierNode(String),
    UnitNode(Box<Ast>),
    BinaryNode {
        left: Box<Ast>,
        right: Box<Ast>,
        op: BinaryOperatorKind
    },
    LetNode {
        assignee: String,
        value: Box<Ast>
    },
    IfNode {
        condition: Box<Ast>,
        body: Vec<Box<Ast>>
    },
    WhileNode {
        condition: Box<Ast>,
        body: Vec<Box<Ast>>
    }
}
