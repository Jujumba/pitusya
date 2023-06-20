pub mod parser;

use crate::lexer::tokens::*;

#[derive(Debug, PartialEq)]
pub enum Ast {
    EOF,
    ValueNode(Token),
    IdentifierNode(String),
    UnitNode(Box<Ast>),
    UnaryNode {
        value: Box<Ast>,
        op: OperatorKind,
    },
    BinaryNode {
        left: Box<Ast>,
        right: Box<Ast>,
        op: OperatorKind,
    },
    LetNode {
        assignee: String,
        value: Box<Ast>,
    },
    IfNode {
        condition: Box<Ast>,
        body: Vec<Box<Ast>>,
    },
    WhileNode {
        condition: Box<Ast>,
        body: Vec<Box<Ast>>,
    },
}
