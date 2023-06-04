use crate::lexer::tokens;

#[derive(Debug)]
pub enum Ast {
    ValueNode(tokens::Token),
    BinaryNode {
        left: Box<Ast>,
        right: Box<Ast>,
        op: tokens::BinaryOperator,
    }
}