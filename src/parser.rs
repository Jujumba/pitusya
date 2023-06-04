use crate::ast::Ast;
use crate::lexer::tokens::{Token, OperatorType};
use crate::lexer::next_token;
use crate::input::InputFile;

pub fn parse_expression(input: &mut InputFile) -> Result<Ast, String> {
    let ast = match next_token(input) {
        i @ Token::Identifier(_) => Ast::ValueNode(i),
        l @ Token::Literal(_) => Ast::ValueNode(l),
        e => {
            return Err(format!("Expected identifier or literal, but got `{e:#?}"));
        }
    };
    match next_token(input) {
        Token::Operator(OperatorType::Binary(op)) => Ok(
                Ast::BinaryNode {
                left: Box::new(ast), right: Box::new(match parse_expression(input) { // todo: recursion is extremely slooow!!!
                    e @ Err(_) => return e,
                    Ok(right) => right
                }), op
            }
        ),
        Token::Operator(OperatorType::Semicol) => Ok(ast),
        e => Err(format!("Expected operator or semicolon, but got `{e:#?}`"))
    }
}