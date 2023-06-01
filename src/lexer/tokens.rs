pub const OPERATORS: [char; 6] = [
    '=',
    '+',
    '/',
    '*',
    '-',
    ';',
];

#[derive(Debug)]
pub enum Token {
    Operator(OperatorType), 
    Keyword(KeywordType), 
    Literal(LiteralType),
    Identifier(String), 
    Undefined(String), // For unrecognised characters, such as $, ^ or @
    EOF,
}

#[derive(Debug)]
pub enum OperatorType {
    Asign,
    Comp,
    Add,
    Div,
    Mul,
    Sub,
    Semicol,
}

impl Token { // todo: use From<&str> trait
    pub fn to_operator(value: String) -> Self {
        if value == "=" {
            Token::Operator(OperatorType::Asign)
        } else if value == "==" {
            Token::Operator(OperatorType::Comp)
        } else if value == "+" {
            Token::Operator(OperatorType::Add)
        } else if value == "-" {
            Token::Operator(OperatorType::Sub)
        } else if value == "*" {
            Token::Operator(OperatorType::Mul)
        } else if value == "/" {
            Token::Operator(OperatorType::Div)
        } else if value == ";" {
            Token::Operator(OperatorType::Semicol)
        } else {
            Token::Undefined(String::new())
        }
    }
    pub fn to_keyword(value: String) -> Self {
        if value == "let" { // todo
            Token::Keyword(KeywordType::Let)
        } else if value == "if" {
            Token::Keyword(KeywordType::If)
        } else if value == "while" {
            Token::Keyword(KeywordType::While)
        } else {
            Token::Identifier(value)
        }
    }
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