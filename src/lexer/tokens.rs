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
    Undefined(String), // For unrecognised characters, such as $ or @
    EOF,
}

#[derive(Debug, PartialEq)]
pub enum OperatorKind {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
    Assignment(AssignmentOperator),
    Paren(ParenKind),
    Semicol,
}
#[derive(Debug, PartialEq)]
pub enum ParenKind {
    LParen,     // (
    RParen,     // )
    LCParen,    // {
    RCParen,    // }
    LBracket,   // [
    RBracket,   // ]
}
#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    BWNot,
    UNot,
}
#[derive(Debug, PartialEq)]
pub enum AssignmentOperator {
    Equals,
    PlusEquals,
    MinusEquals,
    MultEquals,
    DivEquals,
    BWLeftShiftEquals,
    BWRightShiftEquals,
    OrEquals,
    AndEquals,
    XorEquals,
    ModuloEquals,
}

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Bigger,
    BiggerOrEq,
    Less,
    LessOrEq,
    Modulo,
    And,
    Xor,
    BWLeftShift,
    BWRightShift,
    Comparision,
}
impl KeywordKind {
    // todo: use From<&str> trait
    pub fn to_keyword(value: &str) -> Option<Self> {
        match value {
            "if" => Some(KeywordKind::If),
            "let" => Some(KeywordKind::Let),
            "while" => Some(KeywordKind::While),
            _ => None,
        }
    }
}
impl OperatorKind {
    pub fn to_operator(op: &str) -> Option<Self> {
        match op {
            "=" => Some(Self::Assignment(AssignmentOperator::Equals)),
            "+=" => Some(Self::Assignment(AssignmentOperator::PlusEquals)),
            "-=" => Some(Self::Assignment(AssignmentOperator::MinusEquals)),
            "*=" => Some(Self::Assignment(AssignmentOperator::MultEquals)),
            "/=" => Some(Self::Assignment(AssignmentOperator::DivEquals)),
            "<<=" => Some(Self::Assignment(AssignmentOperator::BWLeftShiftEquals)),
            ">>=" => Some(Self::Assignment(AssignmentOperator::BWRightShiftEquals)),
            "|=" => Some(Self::Assignment(AssignmentOperator::OrEquals)),
            "&=" => Some(Self::Assignment(AssignmentOperator::AndEquals)),
            "^=" => Some(Self::Assignment(AssignmentOperator::XorEquals)),
            "%=" => Some(Self::Assignment(AssignmentOperator::ModuloEquals)),
            "+" => Some(Self::Binary(BinaryOperator::Addition)),
            "-" => Some(Self::Binary(BinaryOperator::Subtraction)),
            "*" => Some(Self::Binary(BinaryOperator::Multiplication)),
            "/" => Some(Self::Binary(BinaryOperator::Division)),
            ">" => Some(Self::Binary(BinaryOperator::Bigger)),
            ">=" => Some(Self::Binary(BinaryOperator::BiggerOrEq)),
            "<" => Some(Self::Binary(BinaryOperator::Less)),
            "<=" => Some(Self::Binary(BinaryOperator::LessOrEq)),
            "%" => Some(Self::Binary(BinaryOperator::Modulo)),
            "&" => Some(Self::Binary(BinaryOperator::And)),
            "|" => Some(Self::Binary(BinaryOperator::Xor)),
            "<<" => Some(Self::Binary(BinaryOperator::BWLeftShift)),
            ">>" => Some(Self::Binary(BinaryOperator::BWRightShift)),
            "==" => Some(Self::Binary(BinaryOperator::Comparision)),
            "~" => Some(Self::Unary(UnaryOperator::BWNot)),
            "!" => Some(Self::Unary(UnaryOperator::UNot)),
            ";" => Some(Self::Semicol),
            "(" => Some(Self::Paren(ParenKind::LCParen)),
            ")" => Some(Self::Paren(ParenKind::RParen)),
            "{" => Some(Self::Paren(ParenKind::LCParen)),
            "}" => Some(Self::Paren(ParenKind::RCParen)),
            "[" => Some(Self::Paren(ParenKind::LBracket)),
            "]" => Some(Self::Paren(ParenKind::RBracket)),
            _ => None,
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
