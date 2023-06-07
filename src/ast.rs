pub mod parser;

use crate::lexer::tokens;

#[derive(Debug, PartialEq)]
pub enum Ast {
    ValueNode(tokens::Token),
    IdentifierNode(String),
    UnitNode(Box<Ast>),
    BinaryNode {
        left: Box<Ast>,
        right: Box<Ast>,
        op: tokens::OperatorKind,
    },
    LetNode {
        assignee: Box<Ast>,
        value: Box<Ast>
    },
    IfNode {
        condition: Box<Ast>,
        body: Vec<Box<Ast>>,
    },
    WhileNode {
        condition: Box<Ast>,
        body: Vec<Box<Ast>>,
    }
}
