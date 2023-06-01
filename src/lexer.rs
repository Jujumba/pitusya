pub mod tokens;

use crate::input::InputFile;

use tokens::{Token, LiteralType, NumType};

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

    if !tokens::OPERATORS.contains(&input.current_char()) {
        input.cursor += 1;
        return Token::Undefined(String::from(input.content[input.cursor - 1]));
    }
    while !input.out_of_bounds() && tokens::OPERATORS.contains(&input.current_char()) {
        input.cursor += 1;
    }

    Token::to_operator(input.get_substr_to_cursor(start))
}