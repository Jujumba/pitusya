pub mod ast;
pub mod codegen;
pub mod input;
pub mod lexer;
pub mod pass;

#[macro_export]
macro_rules! abort {
    () => {
        ::std::process::exit(0x_48_41_50_50_59_42_44_41_59_i128 as i32)
    };
    ($msg:expr) => {{
        eprintln!($msg);
        ::std::process::exit(0x_48_41_50_50_59_42_44_41_59_i128 as i32)
    }};
    ($msg:expr, $($args:expr),*) => {{
        eprintln!($msg, $($args),*);
        ::std::process::exit(0x_48_41_50_50_59_42_44_41_59_i128 as i32)
    }};
}
#[macro_export]
macro_rules! abort_if_not {
    ($cond:expr, $msg: expr) => {
        if ! $cond {
            eprintln!($msg);
            ::std::process::exit(0x_48_41_50_50_59_42_44_41_59_i128 as i32);
        }
    };
    ($cond:expr, $msg: expr, $($p:expr),*) => {
        if ! $cond {
            eprintln!($msg, $($p),*);
            ::std::process::exit(0x_48_41_50_50_59_42_44_41_59_i128 as i32);
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::ast::{parser, Ast};
    use crate::input::CursoredFile;
    use crate::lexer::next_token;
    use crate::lexer::tokens::*;
    #[test]
    #[ignore = "Compiler doesn't panic anymore, instead it exits with a non-zero value (what is untestable)."]
    fn test_bad_input() {
        let mut bad = CursoredFile::new("1 = 1;".to_string());
        parser::parse(&mut bad);
    }
    #[test]
    fn test_let_expr() {
        let mut parse = CursoredFile::new(
            "fn main() {
            let pitusya = \"cool\";
        }"
            .to_string(),
        );
        parser::parse(&mut parse);
    }
    #[test]
    #[ignore = "I've added functions and user is enforced to write their code in them"]
    fn test_lexer() {
        let mut tok_seq = CursoredFile::new(String::from("==<=|"));
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
    #[ignore = "Okey, now all my tests are useless"]
    fn test_while_expression_parsing() {
        let mut input = CursoredFile::new(String::from(
            "while 1 == 1; {
            let hello = \"world\";
        }",
        ));
        let ast = parser::parse(&mut input);
        assert!(matches!(ast[0], Ast::WhileNode { .. }));
    }
    #[test]
    #[ignore = "Even though my parser is testable, it would be so tedious to write tests further"]
    fn test_if_expression_parsing() {
        let mut input = CursoredFile::new(String::from(
            "if 1 == 2; {
                let wow = \"uWu\";
            }",
        ));
        let ast = parser::parse(&mut input);
        assert!(matches!(ast[0], Ast::IfNode { .. }))
    }
    #[test]
    #[ignore = "Does all that CI configuration headache worth it, since I dont need to test anymore?.."]
    fn one_more_thing() {}
}
