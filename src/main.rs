use std::process::ExitCode;

use clap::Parser;

use pitusya::ast::parser;
use pitusya::ast::Ast;
use pitusya::codegen::Cg;
use pitusya::input::{CursoredFile, Cli};
use pitusya::pass;
use pitusya::abort;

fn main() -> ExitCode {
    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(e) => {
            e.print().unwrap();
            abort!() // I need my special exit code ^.^
        }
    };
    let mut input = CursoredFile::new(cli);

    let mut cg = Cg::default();

    let asts: Vec<Ast> = parser::parse(&mut input);
    pass::pipeline(&asts);
    asts.into_iter().for_each(|ast| cg.codegen(ast));

    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    ExitCode::from(cg.exec() as u8)
}
