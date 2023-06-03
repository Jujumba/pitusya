use pitusya::input::InputFile;
use pitusya::lexer::{next_token, tokens};

fn main() {
    let mut titi = InputFile {
        content: ">>11<af".chars().collect(),
        cursor: 0,
    };
    loop {
        let token = next_token(&mut titi);
        println!("{token:?}");
        if let tokens::Token::EOF = token {
            break;
        }
    }
}
