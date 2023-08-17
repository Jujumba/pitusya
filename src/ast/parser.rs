use super::Proto;

use crate::abort;
use crate::ast::Ast;
use crate::input::CursoredFile;
use crate::lexer::next_token;
use crate::lexer::tokens::{BinaryOperatorKind, KeywordKind, OperatorKind, TokenKind};

macro_rules! abort_syntax_analysis {
    ($pos: expr) => {
        abort!("Compilation error at position {}", $pos)
    };
    ($pos:expr, $msg:expr) => {
        abort!("Compilation error at position {}\n\t{}", $pos, $msg)
    };
    ($pos: expr, $expected: expr, $error: expr) => {
        abort!(
            "Compilation error at position {}:\n\tExpected {}, but got {:?}",
            $pos,
            $expected,
            $error
        )
    };
}

pub fn parse(input: &mut CursoredFile) -> Vec<Ast> {
    let mut ast = Vec::new();
    loop {
        match next_token(input).kind {
            TokenKind::Keyword(KeywordKind::Fn) => ast.push(Ast::FunctionNode {
                proto: parse_prototype(input, true),
                body: parse_block(input),
            }),
            TokenKind::Keyword(KeywordKind::Extern) => {
                ast.push(Ast::ExternNode(parse_prototype(input, true)));
            }
            TokenKind::EOF => break,
            e => abort_syntax_analysis!(input.get_cursor(), "`extern` or `fn`", e),
        }
    }
    ast
}
fn parse_prototype(input: &mut CursoredFile, definition: bool) -> Proto {
    let name_token = next_token(input);
    let name = match name_token.kind {
        TokenKind::Identifier(name) => name,
        e => abort_syntax_analysis!(input.get_cursor(), "function's name in its definition", e),
    };

    match next_token(input).kind {
        TokenKind::Operator(OperatorKind::LParen) => (),
        e => abort_syntax_analysis!(input.get_cursor(), "`(`", e),
    }

    let mut args = Vec::<Ast>::new();
    let mut t = next_token(input);

    while t.kind != TokenKind::Operator(OperatorKind::RParen) {
        match t.kind {
            TokenKind::Identifier(_) if name == "main" => {
                abort_syntax_analysis!(input.get_cursor(), "Main function cannot accept parameters!");
            }
            TokenKind::Identifier(param) if definition => args.push(Ast::IdentifierNode(param)),
            _ if !definition => {
                input.move_back_cursor(t.len);
                args.push(parse_expression(input));
            }
            e => abort_syntax_analysis!(input.get_cursor(), "an identifier", e),
        }
        match next_token(input).kind {
            TokenKind::Operator(OperatorKind::Coma) => {
                t = next_token(input);
                continue;
            }
            TokenKind::Operator(OperatorKind::RParen) => break,
            e => abort_syntax_analysis!(input.get_cursor(), "a coma or `)`", e),
        }
    }
    Proto { name, args }
}
fn parse_block(input: &mut CursoredFile) -> Vec<Ast> {
    let curly = next_token(input);
    if curly.kind != TokenKind::Operator(OperatorKind::LCurly) {
        abort_syntax_analysis!(input.get_cursor(), "`{`", curly);
    }
    let mut body = vec![];
    loop {
        let t = next_token(input);
        match t.kind {
            TokenKind::Keyword(KeywordKind::If) => body.push(Ast::IfNode {
                condition: Box::new(parse_expression(input)),
                body: parse_block(input),
            }),
            TokenKind::Keyword(KeywordKind::While) => body.push(Ast::WhileNode {
                condition: Box::new(parse_expression(input)),
                body: parse_block(input),
            }),
            TokenKind::Keyword(KeywordKind::Let) => body.push(parse_let_expr(input)),
            TokenKind::Identifier(_) | TokenKind::Literal(_) | TokenKind::Operator(OperatorKind::LParen) => {
                input.move_back_cursor(t.len);
                body.push(parse_expression(input));
            }
            TokenKind::Keyword(KeywordKind::Ret) => body.push(Ast::RetNode(Box::new(parse_expression(input)))),
            TokenKind::Operator(OperatorKind::RCurly) => break,
            _ => abort_syntax_analysis!(input.get_cursor(), "Unexpected token"),
        }
    }
    body
}
fn parse_expression(input: &mut CursoredFile) -> Ast {
    let ast = fetch_lhs(input, "an identifier or literal");
    let token = next_token(input);
    if let TokenKind::Operator(op) = token.kind {
        match op {
            OperatorKind::Binary(BinaryOperatorKind::Assigment) if matches!(ast, Ast::ValueNode(_)) => {
                abort_syntax_analysis!(input.get_cursor(), format!("Cannot assign to the const-value of {ast:?}"));
            }
            OperatorKind::Binary(op) => Ast::BinaryNode {
                left: Box::new(ast),
                right: Box::new(parse_expression(input)),
                op,
            },
            _ => {
                input.move_back_cursor(token.len);
                ast
            }
        }
    } else {
        input.move_back_cursor(token.len);
        ast
    }
}
fn parse_unit_expr(input: &mut CursoredFile) -> Ast {
    let ast = fetch_lhs(input, "an identifier or literal");
    match next_token(input).kind {
        TokenKind::Operator(op) => match op {
            OperatorKind::Binary(op) => Ast::BinaryNode {
                left: Box::new(ast),
                right: Box::new(parse_unit_expr(input)),
                op,
            },
            OperatorKind::RParen => ast,
            e => abort_syntax_analysis!(input.get_cursor(), "a binary operator or `)`", e),
        },
        e => abort_syntax_analysis!(input.get_cursor(), "`)`", e),
    }
}
fn fetch_lhs(input: &mut CursoredFile, expected: &str) -> Ast {
    let lhs_token = next_token(input);
    match lhs_token.kind {
        TokenKind::Identifier(_) => {
            input.move_back_cursor(lhs_token.len); // todo: !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
            fetch_ident_or_call(input)
        }
        TokenKind::Literal(l) => Ast::ValueNode(l),
        TokenKind::Operator(OperatorKind::LParen) => Ast::UnitNode(Box::new(parse_unit_expr(input))),
        e => abort_syntax_analysis!(input.get_cursor(), expected, e),
    }
}
fn fetch_ident_or_call(input: &mut CursoredFile) -> Ast {
    let name_token = next_token(input);
    let name = match name_token.kind {
        TokenKind::Identifier(i) => i,
        e => abort_syntax_analysis!(input.get_cursor(), "an identifier", e),
    };
    let paren = next_token(input);
    input.move_back_cursor(paren.len);
    if !matches!(paren.kind, TokenKind::Operator(OperatorKind::LParen)) {
        return Ast::IdentifierNode(name);
    };
    input.move_back_cursor(name_token.len);
    Ast::CallNode(parse_prototype(input, false))
}
fn parse_let_expr(input: &mut CursoredFile) -> Ast {
    let token = next_token(input);
    match token.kind {
        TokenKind::Identifier(assignee) => match next_token(input).kind {
            TokenKind::Operator(OperatorKind::Binary(BinaryOperatorKind::Assigment)) => Ast::LetNode {
                assignee,
                value: Box::new(parse_expression(input)),
            },
            e => abort_syntax_analysis!(input.get_cursor(), "`=`", e),
        },
        e => abort_syntax_analysis!(input.get_cursor(), "an identifier", e),
    }
}
