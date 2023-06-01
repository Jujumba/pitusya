use pitusya::{input::InputFile, lexer::next_token};
use pitusya::lexer::tokens::Token;
fn main() {
    let mut input = InputFile {
        content: "1.1 .1".chars().collect(),
        cursor: 0
    };
    loop {
        let token = next_token(&mut input);
        println!("{token:?}");
        if let Token::EOF = token {
            break;
        }
    }
}