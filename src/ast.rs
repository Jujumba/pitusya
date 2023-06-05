pub mod parser;

use crate::lexer::tokens;

#[derive(Debug, PartialEq)]
pub enum Ast {
    ValueNode(tokens::Token),
    BinaryNode {
        left: Box<Ast>,
        right: Box<Ast>,
        op: tokens::BinaryOperator,
    },
    LetNode {
        assignee: Box<Ast>,
        value: Box<Ast>
    }
}
