use crate::ast::Ast;
use crate::input::InputFile;
use crate::lexer::next_token;
use crate::lexer::tokens::*;

pub fn parse(input: &mut InputFile) -> Result<Ast, String> {
    let t = next_token(input);
    match &t.kind {
        TokenKind::Keyword(KeywordKind::Let) => parse_let_expr(input),
        TokenKind::Identifier(_) | TokenKind::Literal(_) | TokenKind::Operator(OperatorKind::Paren(ParenKind::LParen)) => {
            input.move_back_cursor(t.len);
            parse_expression(input)
        }
        _ => Err(format!("Unsupported!"))
    }
}
pub fn parse_expression(input: &mut InputFile) -> Result<Ast, String> {
    let t = next_token(input);
    let ast = match t.kind {
        TokenKind::Literal(_) => Ast::ValueNode(t),
        TokenKind::Identifier(i) => Ast::IdentifierNode(i),
        TokenKind::Operator(OperatorKind::Paren(ParenKind::LParen)) => {
            let ast = Ast::UnitNode(Box::new(parse_expression(input)?));
            let t = next_token(input);
            match &t.kind {
                TokenKind::Operator(OperatorKind::Paren(ParenKind::RParen)) => return Err("Unexpected `(`".to_string()),
                _ => {
                    input.move_back_cursor(t.len);
                    ast
                }
            }
        } 
        e => return Err(format!("Expected identifier or literal, but got `{e:?}")),
    };
    match next_token(input).kind {
        TokenKind::Operator(OperatorKind::Semicol) | TokenKind::Operator(OperatorKind::Paren(ParenKind::RParen)) => Ok(ast),
        TokenKind::Operator(op @ OperatorKind::Binary(_)) => Ok(
            Ast::BinaryNode {
                left: Box::new(ast),
                right: Box::new(parse_expression(input)?),
                op,
            }
        ),
        e => Err(format!("Expected operator or semicolon, but got `{e:?}`")),
    }
}
pub fn parse_let_expr(input: &mut InputFile) -> Result<Ast, String> {
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