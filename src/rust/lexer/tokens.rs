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
    Undefined(char), // For unrecognised characters, such as $ or @
    EOF
}

#[derive(Debug, PartialEq)]
pub enum OperatorKind {
    LParen,    // (
    RParen,    // )
    LCurly,    // {
    RCurly,    // }
    LBracket,  // [
    RBracket,  // ]
    Semicol,   // ;
    Binary(BinaryOperatorKind)
}
#[derive(Debug, PartialEq)]
pub enum BinaryOperatorKind {
    Assigment, // =
    Comparision,    // ==
    Addition,       // +
    Subtraction,    // -
    Multiplication, // *
    Division,       // /
    Bigger,         // >
    BiggerOrEq,     // >=
    Less,           // <
    LessOrEq        // <=
}
impl TryFrom<&str> for KeywordKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "if" => Ok(KeywordKind::If),
            "let" => Ok(KeywordKind::Let),
            "while" => Ok(KeywordKind::While),
            "fn" => Ok(KeywordKind::Fn),
            _ => Err(())
        }
    }
}
impl TryFrom<&str> for OperatorKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "<=" => Ok(Self::Binary(BinaryOperatorKind::LessOrEq)),
            ">=" => Ok(Self::Binary(BinaryOperatorKind::BiggerOrEq)),
            "==" => Ok(Self::Binary(BinaryOperatorKind::Comparision)),
            "+" => Ok(Self::Binary(BinaryOperatorKind::Addition)),
            "-" => Ok(Self::Binary(BinaryOperatorKind::Subtraction)),
            "*" => Ok(Self::Binary(BinaryOperatorKind::Multiplication)),
            "/" => Ok(Self::Binary(BinaryOperatorKind::Division)),
            "<" => Ok(Self::Binary(BinaryOperatorKind::Less)),
            ">" => Ok(Self::Binary(BinaryOperatorKind::Bigger)),
            "=" => Ok(Self::Binary(BinaryOperatorKind::Assigment)),
            ";" => Ok(Self::Semicol),
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
    Fn
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Num(f64),
    Str(String)
}
