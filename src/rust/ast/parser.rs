use crate::abort_syntax_analysis;
use crate::ast::Ast;
use crate::input::InputFile;
use crate::lexer::next_token;
use crate::lexer::tokens::*;

pub fn parse(input: &mut InputFile) -> Ast {
    let t = next_token(input);
    match &t.kind {
        TokenKind::Keyword(KeywordKind::Let) => parse_let_expr(input),
        TokenKind::Identifier(_) | TokenKind::Literal(_) | TokenKind::Operator(OperatorKind::LParen) => {
            input.move_back_cursor(t.len);
            parse_expression(input)
        }
        TokenKind::Keyword(KeywordKind::If) => Ast::IfNode {
            condition: Box::new(parse_expression(input)),
            body: parse_block(input),
        },
        TokenKind::Keyword(KeywordKind::While) => Ast::WhileNode {
            condition: Box::new(parse_expression(input)),
            body: parse_block(input),
        },
        TokenKind::EOF => Ast::EOF,
        TokenKind::Operator(OperatorKind::Semicol) => parse(input),
        _ => {
            abort_syntax_analysis!(input.get_cursor());
        }
    }
}
fn parse_block(input: &mut InputFile) -> Vec<Ast> {
    let curly = next_token(input);
    if curly.kind != TokenKind::Operator(OperatorKind::LCurly) {
        abort_syntax_analysis!(input.get_cursor(), "`}}`", curly);
    }
    let mut body = vec![];
    let mut t = next_token(input);
    while t.kind != TokenKind::Operator(OperatorKind::RCurly) {
        input.move_back_cursor(t.len); // todo: bad code ^.^
        body.push(parse(input)); // todo: fetching the same token twice
        t = next_token(input);
        if t.kind == TokenKind::EOF {
            abort_syntax_analysis!(input.get_cursor(), "`}}`", t);
        }
    }
    body
}
fn parse_expression(input: &mut InputFile) -> Ast {
    let ast = fetch_lhs(input, "an identifier or literal");
    match next_token(input).kind {
        TokenKind::Operator(op) => match op {
            OperatorKind::Binary(BinaryOperatorKind::Assigment) if matches!(ast, Ast::ValueNode(_)) => {
                abort_syntax_analysis!(input.get_cursor(), format!("Cannot assign to the const-value of {ast:?}"));
            }
            OperatorKind::Binary(op) => Ast::BinaryNode {
                left: Box::new(ast),
                right: Box::new(parse_expression(input)),
                op,
            },
            OperatorKind::Semicol => ast,
            e => {
                abort_syntax_analysis!(input.get_cursor(), "a binary operator or semicolon", e);
            }
        },
        e => {
            abort_syntax_analysis!(input.get_cursor(), "a binary operator or semicolon", e);
        }
    }
}
fn parse_unit_expr(input: &mut InputFile) -> Ast {
    let ast = fetch_lhs(input, "an identifier or literal");
    match next_token(input).kind {
        TokenKind::Operator(op) => match op {
            OperatorKind::Binary(op) => Ast::BinaryNode {
                left: Box::new(ast),
                right: Box::new(parse_unit_expr(input)),
                op,
            },
            OperatorKind::RParen => ast,
            e => {
                abort_syntax_analysis!(input.get_cursor(), "a binary operator or `)`", e);
            }
        },
        e => {
            abort_syntax_analysis!(input.get_cursor(), "`)`", e);
        }
    }
}
fn fetch_lhs(input: &mut InputFile, expected: &str) -> Ast {
    match next_token(input).kind {
        TokenKind::Identifier(i) => Ast::IdentifierNode(i),
        TokenKind::Literal(l) => Ast::ValueNode(l),
        TokenKind::Operator(OperatorKind::LParen) => Ast::UnitNode(Box::new(parse_unit_expr(input))),
        e => {
            abort_syntax_analysis!(input.get_cursor(), expected, e);
        }
    }
}
fn parse_let_expr(input: &mut InputFile) -> Ast {
    let token = next_token(input);
    match token.kind {
        TokenKind::Identifier(assignee) => match next_token(input).kind {
            TokenKind::Operator(OperatorKind::Binary(BinaryOperatorKind::Assigment)) => Ast::LetNode {
                assignee,
                value: Box::new(parse_expression(input)),
            },
            e => {
                abort_syntax_analysis!(input.get_cursor(), "`=`", e);
            }
        },
        e => {
            abort_syntax_analysis!(input.get_cursor(), "an identifier", e);
        }
    }
}
