pub mod ast;
pub mod input;
pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    use crate::{input, parser};
    #[test]
    fn test_parsing() {
        let mut input = input::InputFile {
            content: "1 + 1 * 2;".chars().collect(),
            cursor: 0,
        };
        let mut input_clone = input.clone();
        assert_eq!(
            parser::parse_expression_iter(&mut input),
            parser::parse_expression_recurs(&mut input_clone)
        );
    }
}
