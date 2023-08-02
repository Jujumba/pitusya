pub mod parser;

use crate::lexer::tokens::{BinaryOperatorKind, LiteralKind};

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
        body: Vec<Ast>
    },
    WhileNode {
        condition: Box<Ast>,
        body: Vec<Ast>
    },
    CallNode(Proto),
    ExternNode(Proto),
    FunctionNode {
        proto: Proto,
        body: Vec<Ast>
    },
    RetNode(Box<Ast>)
}
#[derive(Debug, PartialEq)]
pub struct Proto {
    pub(crate) name: String,
    pub(crate) args: Vec<Ast>
}
