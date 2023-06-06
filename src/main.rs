use pitusya::input::InputFile;

fn main() {
    let mut titi = InputFile::new(String::from("1 == 1;"));
    println!("{:#?}", pitusya::ast::parser::parse(&mut titi));
}
