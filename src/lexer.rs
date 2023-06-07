pub mod tokens;

use lazy_static::lazy_static;
use regex::Regex;
use tokens::{KeywordKind, LiteralKind, NumType, OperatorKind, Token, TokenKind};

use crate::input::InputFile;

type Handler = dyn Fn(&str) -> TokenKind + Sync;

lazy_static! {
    static ref SPEC: Vec<(Regex, Box<Handler>)> = vec! {
        (Regex::new(r"[0-9]+").unwrap(), Box::new(
            |s| {
                TokenKind::Literal(LiteralKind::Num(NumType::Int(s.parse().unwrap())))
            }
        )),
        (Regex::new("\"[a-zA-Z0-0]+\"").unwrap(), Box::new(
            |s| {
                TokenKind::Literal(LiteralKind::Str(s.into()))
            }
        )),
        (Regex::new(r"[a-zA-Z0-9]+").unwrap(), Box::new(
            |s| {
                match KeywordKind::to_keyword(s) {
                    Some(keyword) => TokenKind::Keyword(keyword),
                    None => TokenKind::Identifier(s.into())
                }
            }
        )),
        (Regex::new(r"<<=|>>=|<=|>=|\+=|\-=|\*=|/=|\|=|\^=|&=|%=|==|<<|>>|=|\+|-|\*|/|%|&|\^|\||~|!|<|>|;|\(|\)|[|]|\{|\}").unwrap(), Box::new(
            |s| {
                match OperatorKind::to_operator(s) {
                    Some(operator) => TokenKind::Operator(operator),
                    None => TokenKind::Undefined(s.chars().next().unwrap())
                }
            }
        ))
    };
}

pub fn next_token(input: &mut InputFile) -> Token {
    if input.out_of_bounds() {
        return Token {
            kind: TokenKind::EOF,
            len: 0,
        };
    }
    input.skip_spaces();
    let content = input.content.iter().collect::<String>();
    for (regex, closure) in SPEC.iter() {
        match regex.find_at(&content, input.cursor) {
            Some(m) if m.start() == input.cursor => {
                let len = m.len();
                input.move_cursor(len);
                let kind = closure(m.as_str());
                return Token { kind, len };
            }
            _ => (),
        }
    }
    let c = input.current_char();
    input.move_cursor(1);
    Token {
        kind: TokenKind::Undefined(c),
        len: 1,
    }
}
