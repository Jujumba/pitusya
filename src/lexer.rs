pub mod tokens;
pub use tokens::Token;

use crate::input::InputFile;

use tokens::{LiteralType, NumType};

use self::tokens::OperatorType;

pub fn next_token(input: &mut InputFile) -> Token {
    if input.out_of_bounds() {
        return Token::EOF;
    }

    input.skip_spaces();

    let start = input.cursor;

    if input.current_char().is_numeric() { 
        loop {
            input.cursor += 1;
            if input.out_of_bounds() || !input.current_char().is_numeric() {
                break;
            }
        } 
        let numeric = input.get_substr_to_cursor(start);
        return Token::Literal(LiteralType::Num(
            NumType::Int(numeric.parse().unwrap())
        ))
    }

    if input.current_char().is_alphanumeric() { // starts always with an alphabetic character
        loop {
            input.cursor += 1;
            if input.out_of_bounds() || !input.current_char().is_alphanumeric()  {
                break;
            }
        }
        let str = input.get_substr_to_cursor(start);
        return Token::to_keyword(str);
    }

    // todo: ugly
    let mut op = input.current_char().to_string();
    while OperatorType::is_operator(&op) {
        input.cursor += 1;
        if input.out_of_bounds() {
            break;
        }
        op.push(input.current_char())
    }
    if !input.out_of_bounds() {
        op.pop();
    }

    match OperatorType::to_operator(&op) {
        Some(op) => Token::Operator(op),
        None => Token::Undefined(op)
    }
}