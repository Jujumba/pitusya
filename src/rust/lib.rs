pub mod ast;
pub mod codegen;
pub mod input;
pub mod lexer;

#[macro_export]
macro_rules! abort_syntax_analysis {
    ($pos: expr) => {
        eprintln!("Compilation error at position {}", $pos);
        std::process::exit(18);
    };
    ($pos:expr, $msg:expr) => {
        eprintln!("Compilation error at position {}\n\t{}", $pos, $msg);
        std::process::exit(18);
    };
    ($pos: expr, $expected: expr, $error: expr) => {
        eprintln!(
            "Compilation error at position {}:\n\tExpected {}, but got {:?}",
            $pos, $expected, $error
        );
        std::process::exit(18);
    };
}

#[cfg(test)]
mod tests {
    use crate::ast::{parser, Ast};
    use crate::input::InputFile;
    use crate::lexer::next_token;
    use crate::lexer::tokens::*;
    #[test]
    #[ignore = "Compiler doesn't panic anymore, instead it exits with a non-zero value (what is untestable)."]
    fn test_bad_input() {
        let mut bad = InputFile::new("1 = 1;".to_string());
        parser::parse(&mut bad);
    }
    #[test]
    fn test_let_expr() {
        let mut parse = InputFile::new("let pitusya = \"cool\";".to_string());
        parser::parse(&mut parse);
    }
    #[test]
    fn test_lexer() {
        let mut tok_seq = InputFile::new(String::from("==<=|"));
        assert_eq!(
            next_token(&mut tok_seq).kind,
            TokenKind::Operator(OperatorKind::Binary(BinaryOperatorKind::Comparision))
        );
        assert_eq!(
            next_token(&mut tok_seq).kind,
            TokenKind::Operator(OperatorKind::Binary(BinaryOperatorKind::LessOrEq))
        );
        assert_eq!(next_token(&mut tok_seq).kind, TokenKind::Undefined('|'));
    }
    #[test]
    fn test_while_expression_parsing() {
        let mut input = InputFile::new(String::from(
            "while 1 == 1; {
            let hello = \"world\";
        }",
        ));
        let ast = parser::parse(&mut input);
        assert!(matches!(ast, Ast::WhileNode { .. }));
    }
    #[test]
    fn test_if_expression_parsing() {
        let mut input = InputFile::new(String::from(
            "if 1 == 2; {
                let wow = \"uWu\";
            }",
        ));
        let ast = parser::parse(&mut input);
        assert!(matches!(ast, Ast::IfNode { .. }))
    }
}
