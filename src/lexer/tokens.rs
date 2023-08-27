#[derive(Debug, PartialEq)]
pub struct Token {
    pub(crate) kind: TokenKind,
    pub(crate) len: usize,
    pub(crate) start: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    Operator(OperatorKind),
    Keyword(KeywordKind),
    Literal(LiteralKind),
    Identifier(String),
    Undefined(char),
    EOF
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperatorKind {
    LParen,   // (
    RParen,   // )
    LCurly,   // {
    RCurly,   // }
    LBracket, // [
    RBracket, // ]
    Semicol,  // ;
    Coma,     // ,
    Binary(BinaryOperatorKind)
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BinaryOperatorKind {
    Assigment,      // =
    Addition,       // +
    Subtraction,    // -
    Multiplication, // *
    Division,       // /
    Comparision(ComparisionOpKind)
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ComparisionOpKind {
    Equals,         // ==
    NeEq,           // !=
    Bigger,         // >
    BiggerOrEq,     // >=
    Less,           // <
    LessOrEq        // <=
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KeywordKind {
    If,
    Let,
    While,
    Fn,
    Extern,
    Ret
}

#[derive(Clone, Debug, PartialEq)]
pub enum LiteralKind {
    Num(f64),
    Str(String)
}
impl Token {
    pub fn eof(start: usize) -> Self {
        Self {
            kind: TokenKind::EOF,
            len: 0,
            start,
        }
    }
    pub fn undefined(c: char, start: usize) -> Self {
        Self {
            kind: TokenKind::Undefined(c),
            start,
            len: 1
        }
    }
}
impl TryFrom<&str> for KeywordKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "if" => Ok(KeywordKind::If),
            "let" => Ok(KeywordKind::Let),
            "while" => Ok(KeywordKind::While),
            "fn" => Ok(KeywordKind::Fn),
            "extern" => Ok(KeywordKind::Extern),
            "ret" => Ok(KeywordKind::Ret),
            _ => Err(())
        }
    }
}
impl TryFrom<&str> for OperatorKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "<=" => Ok(Self::Binary(BinaryOperatorKind::Comparision(ComparisionOpKind::LessOrEq))),
            ">=" => Ok(Self::Binary(BinaryOperatorKind::Comparision(ComparisionOpKind::BiggerOrEq))),
            "==" => Ok(Self::Binary(BinaryOperatorKind::Comparision(ComparisionOpKind::Equals))),
            "!=" => Ok(Self::Binary(BinaryOperatorKind::Comparision(ComparisionOpKind::NeEq))),
            "+" => Ok(Self::Binary(BinaryOperatorKind::Addition)),
            "-" => Ok(Self::Binary(BinaryOperatorKind::Subtraction)),
            "*" => Ok(Self::Binary(BinaryOperatorKind::Multiplication)),
            "/" => Ok(Self::Binary(BinaryOperatorKind::Division)),
            "<" => Ok(Self::Binary(BinaryOperatorKind::Comparision(ComparisionOpKind::Less))),
            ">" => Ok(Self::Binary(BinaryOperatorKind::Comparision(ComparisionOpKind::Bigger))),
            "=" => Ok(Self::Binary(BinaryOperatorKind::Assigment)),
            ";" => Ok(Self::Semicol),
            "," => Ok(Self::Coma),
            "(" => Ok(Self::LParen),
            ")" => Ok(Self::RParen),
            "{" => Ok(Self::LCurly),
            "}" => Ok(Self::RCurly),
            "[" => Ok(Self::LBracket),
            "]" => Ok(Self::RBracket),
            _ => Err(())
        }
    }
}