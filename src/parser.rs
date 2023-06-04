use std::collections::VecDeque;

use crate::ast::Ast;
use crate::input::InputFile;
use crate::lexer::next_token;
use crate::lexer::tokens::{BinaryOperator, OperatorType, Token};

pub fn parse_expression_iter(input: &mut InputFile) -> Result<Ast, String> {
    let mut ast_stack: VecDeque<Ast> = VecDeque::new();
    let mut op_stack: VecDeque<BinaryOperator> = VecDeque::new();
    loop {
        match next_token(input) {
            i @ Token::Identifier(_) => ast_stack.push_back(Ast::ValueNode(i)),
            l @ Token::Literal(_) => ast_stack.push_back(Ast::ValueNode(l)),
            e => return Err(format!("Expected identifier ot literal, but got `{e:?}`")),
        }
        match next_token(input) {
            Token::Operator(OperatorType::Semicol) => break,
            Token::Operator(OperatorType::Binary(bin)) => op_stack.push_back(bin),
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
    let ast = match next_token(input) {
        i @ Token::Identifier(_) => Ast::ValueNode(i),
        l @ Token::Literal(_) => Ast::ValueNode(l),
        e => {
            return Err(format!("Expected identifier or literal, but got `{e:#?}"));
        }
    };
    match next_token(input) {
        Token::Operator(OperatorType::Binary(op)) => Ok(Ast::BinaryNode {
            left: Box::new(ast),
            right: Box::new(match parse_expression_recurs(input) {
                // todo: recursion is extremely slooow!!!
                e @ Err(_) => return e,
                Ok(right) => right,
            }),
            op,
        }),
        Token::Operator(OperatorType::Semicol) => Ok(ast),
        e => Err(format!("Expected operator or semicolon, but got `{e:#?}`")),
    }
}
