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
        body: Vec<Ast>
    },
    WhileNode {
        condition: Box<Ast>,
        body: Vec<Ast>
    },
    PrototypeNode {
        name: String,
        args: Vec<String>
    },
    FunctionNode {
        proto: Box<Ast>,
        body: Vec<Ast>
    }
}
