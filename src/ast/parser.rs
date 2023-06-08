use crate::ast::Ast;
use crate::input::InputFile;
use crate::lexer::next_token;
use crate::lexer::tokens::*;

macro_rules! expect {
    ($expected: expr, $e: expr) => {
        Err(format!("Expected {}, but got {:?}", $expected, $e))
    };
}

pub fn parse(input: &mut InputFile) -> Result<Ast, String> {
    let t = next_token(input);
    match &t.kind {
        TokenKind::Keyword(KeywordKind::Let) => parse_let_expr(input),
        TokenKind::Identifier(_)
        | TokenKind::Literal(_)
        | TokenKind::Operator(OperatorKind::LParen) => {
            input.move_back_cursor(t.len);
            parse_expression(input)
        }
        _ => Err(format!(
            "Unexpected token at position {}",
            input.get_cursor()
        )),
    }
}
pub fn parse_expression(input: &mut InputFile) -> Result<Ast, String> {
    let t = next_token(input);
    let ast = match t.kind {
        TokenKind::Literal(_) => Ast::ValueNode(t),
        TokenKind::Identifier(i) => Ast::IdentifierNode(i),
        TokenKind::Operator(OperatorKind::LParen) => {
            let ast = Ast::UnitNode(Box::new(parse_expression(input)?));
            let t = next_token(input);
            match &t.kind {
                TokenKind::Operator(OperatorKind::RParen) => {
                    return Err("Unexpected `)`".to_string())
                }
                _ => {
                    input.move_back_cursor(t.len);
                    ast
                }
            }
        }
        TokenKind::Operator(op @ OperatorKind::UNot) => {
            return Ok(Ast::UnaryNode {
                value: Box::new(parse_expression(input)?),
                op,
            });
        }
        TokenKind::Operator(op @ OperatorKind::BWNot) => {
            return Ok(Ast::UnaryNode {
                value: Box::new(parse_expression(input)?),
                op,
            });
        }
        e => return expect!("identifier or literal", e),
    };
    match next_token(input).kind {
        TokenKind::Operator(OperatorKind::Semicol) | TokenKind::Operator(OperatorKind::RParen) => {
            Ok(ast)
        }
        TokenKind::Operator(op) => Ok(Ast::BinaryNode {
            left: Box::new(ast),
            right: Box::new(parse_expression(input)?),
            op,
        }),
        e => expect!("operator or semicolon", e),
    }
}
pub fn parse_let_expr(input: &mut InputFile) -> Result<Ast, String> {
    let assignee = next_token(input);
    match &assignee.kind {
        TokenKind::Identifier(_) => match next_token(input).kind {
            TokenKind::Operator(OperatorKind::Equals) => Ok(Ast::LetNode {
                assignee: Box::new(Ast::ValueNode(assignee)),
                value: Box::new(parse_expression(input)?),
            }),
            e => expect!("=", e),
        },
        e => expect!("identifier", e),
    }
}
