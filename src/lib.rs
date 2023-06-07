pub mod ast;
pub mod input;
pub mod lexer;

#[cfg(test)]
mod tests {
    use crate::ast::parser;
    use crate::input::InputFile;
    use crate::lexer::next_token;
    use crate::lexer::tokens::*;
    #[test]
    #[should_panic]
    fn test_bad_input() {
        let mut incorrect = InputFile::new("1 + 1;".to_string());
        parser::parse(&mut incorrect).unwrap();
    }
    #[test]
    fn test_let_expr() {
        let mut parse = InputFile::new("let pitusya = \"cool\";".to_string());
        parser::parse(&mut parse).unwrap();
    }
    #[test]
    fn test_lexer() {
        let mut tok_seq = InputFile::new(String::from("==<=|=%==<<="));
        assert_eq!(
            next_token(&mut tok_seq).kind,
            TokenKind::Operator(OperatorKind::Binary(BinaryOperator::Comparision))
        );
        assert_eq!(
            next_token(&mut tok_seq).kind,
            TokenKind::Operator(OperatorKind::Binary(BinaryOperator::LessOrEq))
        );
        assert_eq!(
            next_token(&mut tok_seq).kind,
            TokenKind::Operator(OperatorKind::Assignment(AssignmentOperator::OrEquals))
        );
        assert_eq!(
            next_token(&mut tok_seq).kind,
            TokenKind::Operator(OperatorKind::Assignment(AssignmentOperator::ModuloEquals))
        );
        assert_eq!(
            next_token(&mut tok_seq).kind,
            TokenKind::Operator(OperatorKind::Assignment(AssignmentOperator::Equals))
        );
        assert_eq!(
            next_token(&mut tok_seq).kind,
            TokenKind::Operator(OperatorKind::Assignment(
                AssignmentOperator::BWLeftShiftEquals
            ))
        );
    }
}
