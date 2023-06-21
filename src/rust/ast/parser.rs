use crate::abort_compilation;
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
        },
        TokenKind::Keyword(KeywordKind::If) | TokenKind::Keyword(KeywordKind::While) => {
            let condition = Box::new(parse_expression(input));
            let curly = next_token(input);
            if matches!(curly.kind, TokenKind::Operator(OperatorKind::LCurly)) {
                let mut curly = next_token(input);
                let mut body = vec![];
                while !matches!(curly.kind, TokenKind::Operator(OperatorKind::RCurly)) {
                    input.move_back_cursor(curly.len);
                    body.push(Box::new(parse(input)));
                    curly = next_token(input);
                    if curly.kind == TokenKind::EOF {
                        abort_compilation!(format!("Expected `}}`, but got {:?}", curly));
                    }
                }
                if matches!(t.kind, TokenKind::Keyword(KeywordKind::While)) {
                    Ast::WhileNode { condition, body }
                } else {
                    Ast::IfNode { condition, body }
                }
            } else {
                abort_compilation!(format!("Expected `}}`, but got {:?}", curly));
            }
        },
        TokenKind::EOF => Ast::EOF,
        TokenKind::Operator(OperatorKind::Semicol) => parse(input),
        _ => {
            abort_compilation!(format!("Unexpected token at position {}", input.get_cursor()));
        }
    }
}
pub fn parse_expression(input: &mut InputFile) -> Ast {
    let t = next_token(input);
    let ast = match t.kind {
        TokenKind::Literal(l) => Ast::ValueNode(l),
        TokenKind::Identifier(i) => Ast::IdentifierNode(i),
        TokenKind::Operator(OperatorKind::LParen) => {
            let ast = Ast::UnitNode(Box::new(parse_expression(input)));
            let t = next_token(input);
            match &t.kind {
                TokenKind::Operator(OperatorKind::RParen) => {
                    abort_compilation!("Unexpected `(`");
                }
                _ => {
                    input.move_back_cursor(t.len);
                    ast
                },
            }
        },
        e => {
            abort_compilation!(format!("Expected identifier or literal, but got {:?}", e));
        }
    };
    match next_token(input).kind {
        TokenKind::Operator(OperatorKind::Semicol) | TokenKind::Operator(OperatorKind::RParen) => ast,
        TokenKind::Operator(OperatorKind::Assigment) if matches!(&ast, Ast::ValueNode(_)) => {
            abort_compilation!("Expected an identifier, but got a const-value");
        },
        TokenKind::Operator(op) => Ast::BinaryNode {
            left: Box::new(ast),
            right: Box::new(parse_expression(input)),
            op,
        },
        e => {
            abort_compilation!(format!("Expected operator or semicolon, but got {:?}", e));
        }
    }
}
pub fn parse_let_expr(input: &mut InputFile) -> Ast {
    let token = next_token(input);
    match token.kind {
        TokenKind::Identifier(assignee) => match next_token(input).kind {
            TokenKind::Operator(OperatorKind::Assigment) => Ast::LetNode {
                assignee,
                value: Box::new(parse_expression(input)),
            },
            e => {
                abort_compilation!(format!("Expected `=`, but got {:?}", e));
            }
        },
            e => {
                abort_compilation!(format!("Expected identifier, but got {:?}", e));
            }
    }
}
