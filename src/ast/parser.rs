use std::collections::VecDeque;

use crate::ast::Ast;
use crate::input::InputFile;
use crate::lexer::next_token;
use crate::lexer::tokens::{BinaryOperator, OperatorKind, TokenKind, KeywordKind, AssignmentOperator};

#[allow(dead_code)]
fn parse_expression_iter(input: &mut InputFile) -> Result<Ast, String> {
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
pub fn parse_expression(input: &mut InputFile) -> Result<Ast, String> {
    let t = next_token(input);
    let ast = match t.kind {
        TokenKind::Identifier(_) | TokenKind::Literal(_) => Ast::ValueNode(t),
        e => return Err(format!("Expected identifier or literal, but got `{e:#?}")),
    };
    match next_token(input).kind {
        TokenKind::Operator(OperatorKind::Binary(op)) => Ok(Ast::BinaryNode {
            left: Box::new(ast),
            right: Box::new(parse_expression(input)?),
            op,
        }),
        TokenKind::Operator(OperatorKind::Semicol) => Ok(ast),
        e => Err(format!("Expected operator or semicolon, but got `{e:#?}`")),
    }
}
pub fn parse_let_expr(input: &mut InputFile) -> Result<Ast, String> {
    let t = next_token(input);
    if TokenKind::Keyword(KeywordKind::Let) != t.kind  {
        return Err(format!("Expected `let`, but got `{t:?}`"));
    }
    let assignee = next_token(input);
    match &assignee.kind {
        TokenKind::Identifier(_) => {
            match next_token(input).kind {
                TokenKind::Operator(OperatorKind::Assignment(AssignmentOperator::Equals)) => Ok(
                    Ast::LetNode {
                        assignee: Box::new(Ast::ValueNode(assignee)),
                        value: Box::new(parse_expression(input)?)
                    }
                ),
                e => Err(format!("Expected `=`, but got `{e:?}"))
            }
        }
        e => Err(format!("Expected identifier, but got `{e:?}`"))
    }
}