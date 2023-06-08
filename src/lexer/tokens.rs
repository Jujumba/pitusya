#[derive(Debug, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub len: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Operator(OperatorKind),
    Keyword(KeywordKind),
    Literal(LiteralKind),
    Identifier(String),
    Undefined(char), // For unrecognised characters, such as $ or @
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum OperatorKind {
    LParen,         // (
    RParen,         // )
    LCurly,         // {
    RCurly,         // }
    LBracket,       // [
    RBracket,       // ]
    Semicol,        // ;
    Comparision,    // ==
    Equals,         // =
    Addition,       // +
    Subtraction,    // -
    Multiplication, // *
    Division,       // /
    Bigger,         // >
    BiggerOrEq,     // >=
    Less,           // <
    LessOrEq,       // <=
    Modulo,         // %
    Or,             // |
    And,            // &
    Xor,            // ^
    BWLeftShift,    // <<
    BWRightShift,   // >>
    BWNot,          // ~
    UNot,           // !
}
impl TryFrom<&str> for KeywordKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "if" => Ok(KeywordKind::If),
            "let" => Ok(KeywordKind::Let),
            "while" => Ok(KeywordKind::While),
            _ => Err(()),
        }
    }
}
impl TryFrom<&str> for OperatorKind {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "<=" => Ok(Self::LessOrEq),
            "<<" => Ok(Self::BWLeftShift),
            ">>" => Ok(Self::BWRightShift),
            "==" => Ok(Self::Comparision),
            "%" => Ok(Self::Modulo),
            "&" => Ok(Self::And),
            "|" => Ok(Self::Or),
            "^" => Ok(Self::Xor),
            "+" => Ok(Self::Addition),
            "-" => Ok(Self::Subtraction),
            "*" => Ok(Self::Multiplication),
            "/" => Ok(Self::Division),
            "=" => Ok(Self::Equals),
            "<" => Ok(Self::Less),
            ">" => Ok(Self::Bigger),
            "~" => Ok(Self::BWNot),
            "!" => Ok(Self::UNot),
            ";" => Ok(Self::Semicol),
            "(" => Ok(Self::LParen),
            ")" => Ok(Self::RParen),
            "{" => Ok(Self::LCurly),
            "}" => Ok(Self::RCurly),
            "[" => Ok(Self::LBracket),
            "]" => Ok(Self::RBracket),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    If,
    Let,
    While,
}

#[derive(Debug, PartialEq)]
pub enum LiteralKind {
    Num(NumType),
    Str(String),
}

#[derive(Debug, PartialEq)]
pub enum NumType {
    Int(i64),
    Float(f64),
}
