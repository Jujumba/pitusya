use pitusya::*;

fn main() {
    let mut titi = input::InputFile::new(String::from("fn bibix(a) {let b = a + 1; ret b + 2;}"));
    let cc = codegen::Cg::default();
    let asts = pitusya::ast::parser::parse(&mut titi);
    asts.into_iter().for_each(|ast| cc.codegen(ast));
}
