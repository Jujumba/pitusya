pub mod tokens;

use crate::input::InputFile;

use tokens::{Token, OperatorType, KeywordType, LiteralType, NumType};

pub fn next_token(input: &mut InputFile) -> Token {
    if input.out_of_bounds() {
        return Token::Eof;
    }

    input.skip_spaces();

    let start = input.cursor;

    if input.current_char().is_numeric() { 
        let mut floating = false;
        loop {
            if input.current_char() == '.' && floating {
                break;
            } else if input.current_char() == '.' {
                floating = true;
            }
            input.cursor += 1;
            if input.out_of_bounds() || !input.current_char().is_numeric() || !(input.current_char() != '.') {
                break;
            }
        } 
        let numeric = input.get_substr_to_cursor(start);
        return Token::Literal(LiteralType::Num(
            if floating {
                NumType::Float(numeric.parse().unwrap())
            } else {
                NumType::Int(numeric.parse().unwrap())
            }
        ))
    }

    if input.current_char().is_alphanumeric() { // starts always with a alphabetic character
        loop {
            input.cursor += 1;
            if input.out_of_bounds() || !input.current_char().is_alphanumeric()  {
                break;
            }
        }
        let str = input.get_substr_to_cursor(start);
        let token = if str == "let" {
            Token::Keyword(KeywordType::Let)
        } else if str == "if" {
            Token::Keyword(KeywordType::If)
        } else if str == "while" {
            Token::Keyword(KeywordType::While)
        } else {
            Token::Identifier(str)
        };
        return token;
    }

    loop {
        input.cursor += 1;
        if input.out_of_bounds() || input.current_char().is_whitespace() {
            break;
        }
    }
    let op = input.get_substr_to_cursor(start);

    let token = if op == "=" {
        Token::Operator(OperatorType::Asign)
    } else if op == "==" {
        Token::Operator(OperatorType::Comp)
    } else if op == "+" {
        Token::Operator(OperatorType::Add)
    } else if op == "-" {
        Token::Operator(OperatorType::Sub)
    } else if op == "*" {
        Token::Operator(OperatorType::Mul)
    } else if op == "/" {
        Token::Operator(OperatorType::Div)
    } else {
        Token::Undefined
    };
    return token;
}