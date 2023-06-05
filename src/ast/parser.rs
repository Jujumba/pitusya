use std::collections::VecDeque;

use crate::ast::Ast;
use crate::input::InputFile;
use crate::lexer::next_token;
use crate::lexer::tokens::{BinaryOperator, OperatorKind, TokenKind};

pub fn parse_expression_iter(input: &mut InputFile) -> Result<Ast, String> {
    let mut ast_stack: VecDeque<Ast> = VecDeque::new();
    let mut op_stack: VecDeque<BinaryOperator> = VecDeque::new();
    loop {
        let t = next_token(input);
        match &t.kind {
            TokenKind::Identifier(_) | TokenKind::Literal(_) => ast_stack.push_back(Ast::ValueNode(t)),
            e => return Err(format!("Expected identifier ot literal, but got `{e:?}`")),
        }
        let t = next_token(input);
        match t.kind {
            TokenKind::Operator(OperatorKind::Semicol) => break,
            TokenKind::Operator(OperatorKind::Binary(bin)) => op_stack.push_back(bin),
            e => return Err(format!("Expected operator or semicolon, but got `{e:?}`")),
        }
    }
    if op_stack.is_empty() {
        return Ok(ast_stack.pop_front().unwrap());
    }
    let mut ast = Ast::BinaryNode {
        right: Box::new(ast_stack.pop_back().unwrap()),
        left: Box::new(ast_stack.pop_back().unwrap()),
        op: op_stack.pop_back().unwrap(),
    };
    while !ast_stack.is_empty() {
        ast = Ast::BinaryNode {
            left: Box::new(ast_stack.pop_back().unwrap()),
            right: Box::new(ast),
            op: op_stack.pop_back().unwrap(),
        }
    }
    return Ok(ast);
}
pub fn parse_expression_recurs(input: &mut InputFile) -> Result<Ast, String> {
    let t = next_token(input);
    let ast = match t.kind {
        TokenKind::Identifier(_) | TokenKind::Literal(_) => Ast::ValueNode(t),
        e => {
            return Err(format!("Expected identifier or literal, but got `{e:#?}"));
        }
    };
    match next_token(input).kind {
        TokenKind::Operator(OperatorKind::Binary(op)) => Ok(Ast::BinaryNode {
            left: Box::new(ast),
            right: Box::new(match parse_expression_recurs(input) {
                // todo: recursion is extremely slooow!!!
                e @ Err(_) => return e,
                Ok(right) => right,
            }),
            op,
        }),
        TokenKind::Operator(OperatorKind::Semicol) => Ok(ast),
        e => Err(format!("Expected operator or semicolon, but got `{e:#?}`")),
    }
}
