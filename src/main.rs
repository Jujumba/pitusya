use pitusya::input::InputFile;

fn main() {
    let mut titi = InputFile {
        content: "1;".chars().collect(),
        cursor: 0,
    };
    println!("{:#?}", pitusya::parser::parse_expression_iter(&mut titi));
}
