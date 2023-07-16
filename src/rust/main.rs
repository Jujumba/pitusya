use std::env;

use pitusya::abort;
use pitusya::ast::parser::parse;
use pitusya::ast::Ast;
use pitusya::codegen::Cg;
use pitusya::input;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        abort!("Error: No input file!");
    }
    let input = &args[1];
    let mut input = input::InputFile::new(input).unwrap_or_else(|_| abort!(format!("Error: File {input} doesn't exist!")));
    let mut cg = Cg::default();
    let asts: Vec<Ast> = parse(&mut input);
    asts.into_iter().for_each(|ast| cg.codegen(ast));
}
