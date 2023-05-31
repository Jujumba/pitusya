#[derive(Debug)]
pub enum Token {
    Operator(OperatorType), 
    Keyword(KeywordType), 
    Literal(LiteralType),
    Identifier(String), 
    Undefined, // For unrecognised characters, such as $, ^ or @
    Eof,
}

#[derive(Debug)]
pub enum OperatorType {
    Asign,
    Comp,
    Add,
    Div,
    Mul,
    Sub,
}

#[derive(Debug)]
pub enum KeywordType {
    If,
    Let,
    While,
}

#[derive(Debug)]
pub enum LiteralType {
    Num(NumType),
    Str(String),
}

#[derive(Debug)]
pub enum NumType {
    Int(i64),
    Float(f64),
}