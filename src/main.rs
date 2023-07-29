use std::env;
use std::process::ExitCode;

use pitusya::abort;
use pitusya::ast::parser::parse;
use pitusya::ast::Ast;
use pitusya::codegen::Cg;
use pitusya::pass::PitusyaPassManager;
use pitusya::input::InputFile;

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        abort!("Error: No input file!");
    }

    let file = &args[1];
    let mut input = InputFile::new(file).unwrap_or_else(|_| abort!(format!("Error: File {file} doesn't exist!")));

    let mut cg = Cg::default();
    let pm = PitusyaPassManager;

    let asts: Vec<Ast> = parse(&mut input);
    pm.pipeline(&asts);
    asts.into_iter().for_each(|ast| cg.codegen(ast));

    #[allow(clippy::cast_possible_truncation)]
    ExitCode::from(cg.exec() as u8)
}
