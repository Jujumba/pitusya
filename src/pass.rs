use colored::Colorize;

use crate::abort_if_not;
use crate::ast::Ast;

pub fn pipeline(asts: &[Ast]) {
    for ast in asts {
        if let Ast::FunctionNode { proto, body } = ast {
            terminated(&proto.name, body);
            no_dead_code(&proto.name, body);
        }
    }
}
fn terminated(name: &str, body: &[Ast]) {
    let mut stack = Vec::with_capacity(body.len());
    // todo: may be written better
    let mut counter = 0;
    for ast in body {
        match ast {
            Ast::WhileNode { body, .. } | Ast::IfNode { body, .. } => stack.push(body),
            Ast::RetNode(_) => counter += 1,
            _ => (),
        }
    }
    abort_if_not!(counter == 1, "{}: function {} returns multiple values or returns nothing", "error".bright_red(), name.bright_cyan());
    stack.into_iter().for_each(|block| {
        let count = block.iter().filter(|ast| matches!(ast, Ast::RetNode(_))).count();
        abort_if_not!(count <= 1, "{}: function {} returns multiple values", "error".bright_red(), name.bright_cyan());
    });
}
fn no_dead_code(name: &str, body: &[Ast]) {
    let mut stack = vec![body]; 
    while let Some(body) = stack.pop() {
        body.iter()
            .filter(|ast| matches!(ast, Ast::IfNode { .. } | Ast::WhileNode { .. }))
            .for_each(|body| match body {
                Ast::IfNode { body, .. } | Ast::WhileNode { body, .. } => stack.push(body),
                _ => (),
            });
        abort_if_not!(
            body.iter().skip_while(|ast| !matches!(ast, Ast::RetNode(_))).nth(1).is_none(),
            "Error: unreachable code in function {}",
            name
        );
    }
}