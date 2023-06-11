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
                match KeywordKind::try_from(s) {
                    Ok(keyword) => TokenKind::Keyword(keyword),
                    _ => TokenKind::Identifier(s.into())
                }
            }
        )),
        (Regex::new(r"<<=|>>=|<=|>=|\+=|\-=|\*=|/=|\|=|\^=|&=|%=|==|<<|>>|=|\+|-|\*|/|%|&|\^|\||~|!|<|>|;|\(|\)|[|]|\{|\}").unwrap(), Box::new(
            |s| {
                match OperatorKind::try_from(s) {
                    Ok(operator) => TokenKind::Operator(operator),
                    _ => TokenKind::Undefined(s.chars().next().unwrap())
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
    let content = input.get_str();
    let curs = input.get_cursor();
    for (regex, closure) in SPEC.iter() {
        match regex.find_at(content, curs) {
            Some(m) if m.start() == curs => {
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
