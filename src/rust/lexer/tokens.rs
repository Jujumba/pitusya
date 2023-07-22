#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Operator(OperatorKind),
    Keyword(KeywordKind),
    Literal(LiteralKind),
    Identifier(String),
    Undefined(char),
    EOF
}

#[derive(Debug, PartialEq)]
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
#[derive(Debug, PartialEq)]
pub enum BinaryOperatorKind {
    Assigment,      // =
    Addition,       // +
    Subtraction,    // -
    Multiplication, // *
    Division,       // /
    Comparision(ComparisionOpKind)
}
#[derive(Debug, PartialEq)]
pub enum ComparisionOpKind {
    Equals,         // ==
    Bigger,         // >
    BiggerOrEq,     // >=
    Less,           // <
    LessOrEq        // <=
}
#[allow(clippy::from_over_into)]
impl Into<i32> for ComparisionOpKind {
    fn into(self) -> i32 {
        match self {
            Self::Equals => 1,
            Self::Bigger => 2,
            Self::BiggerOrEq => 3,
            Self::Less => 4,
            Self::LessOrEq => 5
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

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    If,
    Let,
    While,
    Fn,
    Extern,
    Ret
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Num(f64),
    Str(String)
}
impl Token {
    pub fn eof() -> Self {
        Self {
            kind: TokenKind::EOF,
            len: 0
        }
    }
    pub fn undefined(c: char) -> Self {
        Self {
            kind: TokenKind::Undefined(c),
            len: 1
        }
    }
}