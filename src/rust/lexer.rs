pub mod tokens;

use std::sync::OnceLock;

use regex::Regex;
use tokens::{KeywordKind, LiteralKind, OperatorKind, Token, TokenKind};

use crate::input::InputFile;

type Handler = dyn Fn(&str) -> TokenKind + Sync + Send;

static SPEC: OnceLock<Vec<(Regex, Box<Handler>)>> = OnceLock::new();

pub fn next_token(input: &mut InputFile) -> Token {
    input.skip_spaces();
    if input.out_of_bounds() {
        return Token {
            kind: TokenKind::EOF,
            len: 0,
        };
    }
    let content = input.get_str();
    let curs = input.get_cursor();
    for (regex, closure) in get_specification().iter() {
        match regex.find_at(content, curs) {
            Some(m) if m.start() == curs => {
                let len = m.len();
                input.move_cursor(len);
                let kind = closure(m.as_str());
                return Token { kind, len };
            },
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

fn get_specification() -> &'static Vec<(Regex, Box<Handler>)> {
    SPEC.get_or_init(|| {
        vec![
            (
                Regex::new(r"[0-9]+").unwrap(),
                Box::new(|s| TokenKind::Literal(LiteralKind::Num(s.parse().unwrap()))),
            ),
            (
                Regex::new("\"[a-zA-Z0-0]+\"").unwrap(),
                Box::new(|s| TokenKind::Literal(LiteralKind::Str(s.into()))),
            ),
            (
                Regex::new(r"[a-zA-Z0-9]+").unwrap(),
                Box::new(|s| match KeywordKind::try_from(s) {
                    Ok(keyword) => TokenKind::Keyword(keyword),
                    _ => TokenKind::Identifier(s.into()),
                }),
            ),
            (
                Regex::new(r"<<=|>>=|<=|>=|\+=|\-=|\*=|/=|\|=|\^=|&=|%=|==|<<|>>|=|\+|-|\*|/|%|&|\^|\||~|!|<|>|;|\(|\)|[|]|\{|\}").unwrap(),
                Box::new(|s| match OperatorKind::try_from(s) {
                    Ok(operator) => TokenKind::Operator(operator),
                    _ => TokenKind::Undefined(s.chars().next().unwrap()),
                }),
            ),
        ]
    })
}
