pub mod tokens;

use tokens::{Token, KeywordType, OperatorType};

use lazy_static::lazy_static;
use regex::Regex;

use crate::input::InputFile;

lazy_static!(
    static ref KEYWORD_REGEX: Regex = Regex::new(r"[a-zA-Z0-9]+").unwrap();
    static ref NUMBERS_REGEX: Regex = Regex::new(r"[0-9]+").unwrap();
    static ref OPERATORS_REGEX: Regex = Regex::new(r"<<=|>>=|<=|>=|\+=|\-=|\*=|/=|\|=|\^=|&=|%=|<<|>>|=|\+|-|\*|/|%|&|\^|\||~|!|<|>").unwrap();
);

pub fn next_token(input: &mut InputFile) -> Token {
    if input.out_of_bounds() {
        return Token::EOF;
    }
    let content = input.content.iter().collect::<String>();
    match NUMBERS_REGEX.find_at(&content, input.cursor) {
        Some(num) if num.start() == input.cursor => {
            let num = num.as_str();
            input.cursor += num.len(); // todo: repeatable code
            return Token::Literal(tokens::LiteralType::Num(tokens::NumType::Int(num.parse().unwrap())));
        },
        _ => ()
    };
    match KEYWORD_REGEX.find_at(&content, input.cursor) {
        Some(keyword) if keyword.start() == input.cursor => {
            let keyword = keyword.as_str();
            input.cursor += keyword.len();
            return match KeywordType::to_keyword(keyword) {
                Some(keyword) => Token::Keyword(keyword),
                None => Token::Identifier(keyword.to_string())
            };
        }
        _ => ()
    }
    match OPERATORS_REGEX.find_at(&content, input.cursor) {
        Some(operator) if operator.start() == input.cursor => {
            let operator = operator.as_str();
            input.cursor += operator.len();
            return match OperatorType::to_operator(operator) {
                Some(operator) => Token::Operator(operator),
                None => Token::Undefined(operator.to_string())
            }
        }
        _ => () 
    }
    let t  = Token::Undefined(input.current_char().to_string());
    input.cursor += 1;
    t
}