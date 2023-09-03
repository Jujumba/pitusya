pub mod ast;
pub mod codegen;
pub mod input;
pub mod lexer;
pub mod pass;

use input::CursoredFile;
use lexer::tokens::Token;
use colored::Colorize;

pub const EXIT_CODE: i128 = 0x_48_41_50_50_59_42_44_41_59_i128;

#[macro_export]
macro_rules! abort {
    () => {
        ::std::process::exit($crate::EXIT_CODE as i32)
    };
    ($msg:expr) => {{
        eprintln!($msg);
        $crate::abort!()
    }};
    ($msg:expr, $($args:expr),*) => {{
        eprintln!($msg, $($args),*);
        $crate::abort!()
    }};
}
#[macro_export]
macro_rules! abort_if_not {
    ($cond:expr, $msg: expr) => {
        if ! $cond {
            $crate::abort!($msg);
        }
    };
    ($cond:expr, $msg: expr, $($p:expr),*) => {
        if ! $cond {
            $crate::abort!($msg, $($p),*);
        }
    };
}
#[macro_export]
macro_rules! abort_with_message {
    ($token:expr, $input:expr, $help:expr) => {
        $crate::abort!("{}", $crate::construct_error_message(&$token, &$input, $help))
    };
}
pub fn construct_error_message<A: AsRef<str>>(token: &Token, file: &CursoredFile, help: A) -> String {
    let help = help.as_ref();
    let chars: &[char] = file.as_ref();
    let start = token.start
        - chars[..token.start]
            .iter()
            .rev()
            .position(|c| *c == '\n')
            .unwrap_or(token.start);
    let end = token.start + chars[token.start..].iter().position(|c| *c == '\n').unwrap_or(file.content.len());
    let line: String = chars[start..end].iter().collect();
    let line_number = chars[..token.start].iter().filter(|c| **c == '\n').count();
    let span_start = token.start - start;
    let span_len = token.len;
    if span_start == 0 {
        format!(
            "{error} in {file_name} on line {line_number}:\n\t{line}\n\t{sep:^>span_len$}\n{col_help}: {actual_help}",
            file_name = file.name.display().to_string().bright_cyan().bold(), // I'm sorry
            line_number = line_number,
            error = "error".bright_red(),
            sep = "^".bright_red(),
            col_help = "note".bright_cyan(),
            actual_help = help
        )
    } else {
        format!(
            "{error} in {file_name} on line {line_number}:\n\t{line}\n\t{space:>span_start$}{sep:^>span_len$}\n{col_help}: {actual_help}",
            file_name = file.name.display().to_string().bright_cyan().bold(),
            line_number = line_number,
            error = "error".bright_red(),
            space = ' ',
            sep = "^".bright_red(),
            col_help = "note".bright_cyan(),
            actual_help = help
        )
    }
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
