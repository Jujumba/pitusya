use pitusya::{lexer::Token, lexer::next_token};
use pitusya::input::InputFile;

fn main() {
    let mut input = InputFile {
        content: ">>11<=".chars().collect(),
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