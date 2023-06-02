#[derive(Debug)]
pub enum Token {
    Operator(OperatorType), 
    Keyword(KeywordType), 
    Literal(LiteralType),
    Identifier(String), 
    Undefined(String), // For unrecognised characters, such as $ or @
    Semicol,
    EOF,
}
#[derive(Debug)]
pub enum OperatorType {
    Unary(UnaryOperator),
    Binary(BinaryOperator),
    Assignment(AssignmentOperator)
    // todo: special type for semicol?
}
#[derive(Debug)]
pub enum UnaryOperator {
    BWNot, 
    UNot, 
}
#[derive(Debug)]
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
#[derive(Debug)]
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
impl KeywordType { // todo: use From<&str> trait
    pub fn to_keyword(value: &str) -> Option<Self> {
        match value {
            "if" => Some(KeywordType::If),
            "let" => Some(KeywordType::Let),
            "while" => Some(KeywordType::While),
            _ => None
        }
    }
}
impl OperatorType {
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
            _ => None,
        }
    }
    pub fn is_operator(op: &str) -> bool {
        match Self::to_operator(op) {
            Some(_) => true,
            None => false
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