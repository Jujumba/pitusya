pub mod tokens;

use lazy_static::lazy_static;
use regex::Regex;
use tokens::{KeywordType, LiteralType, NumType, OperatorType, Token};

use crate::input::InputFile;

type Handler = dyn Fn(&str) -> Token + Sync;

lazy_static! {
    static ref SPEC: Vec<(Regex, Box<Handler>)> = vec! {
        (Regex::new(r"[0-9]+").unwrap(), Box::new(
            |s| {
                Token::Literal(LiteralType::Num(NumType::Int(s.parse().unwrap())))
            }
        )),
        (Regex::new("\"[a-zA-Z0-0]+\"").unwrap(), Box::new(
            |s| {
                Token::Literal(LiteralType::Str(s.into()))
            }
        )),
        (Regex::new(r"[a-zA-Z0-9]+").unwrap(), Box::new(
            |s| {
                match KeywordType::to_keyword(s) {
                    Some(keyword) => Token::Keyword(keyword),
                    None => Token::Identifier(s.into())
                }
            }
        )),
        (Regex::new(r"<<=|>>=|<=|>=|\+=|\-=|\*=|/=|\|=|\^=|&=|%=|<<|>>|=|\+|-|\*|/|%|&|\^|\||~|!|<|>").unwrap(), Box::new(
            |s| {
                match OperatorType::to_operator(s) {
                    Some(operator) => Token::Operator(operator),
                    None => Token::Undefined(s.into())
                }
            }
        ))
    };
}

pub fn next_token(input: &mut InputFile) -> Token {
    if input.out_of_bounds() {
        return Token::EOF;
    }
    let content = input.content.iter().collect::<String>();
    let mut nearest: Option<regex::Match> = None;
    for (regex, closure) in SPEC.iter() {
        match regex.find_at(&content, input.cursor) {
            Some(m) if m.start() == input.cursor => {
                let s = m.as_str();
                input.move_cursor(s.len());
                return closure(s);
            },
            Some(m) => {
                nearest = match nearest {
                    Some(n) if n.start() > m.start() => Some(m),
                    None => Some(m),
                    _ => nearest
                };
            },
            _ => ()
        }
    }
    let undef_token_string = input.content[input.cursor..nearest.unwrap().start()].iter().collect::<String>();
    input.move_cursor(undef_token_string.len());
    Token::Undefined(undef_token_string)
}
