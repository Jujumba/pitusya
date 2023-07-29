use crate::abort_if_not;
use crate::ast::Ast;

pub struct PitusyaPassManager;
impl PitusyaPassManager {
    pub fn pipeline(&self, asts: &[Ast]) {
        for ast in asts {
            if let Ast::FunctionNode { proto, body } = ast {
                self.terminated(&proto.name, body);
            }
        }
    }
    fn terminated(&self, name: &str, body: &[Ast]) {
        let mut stack = Vec::with_capacity(body.len());
        // todo: may be written better
        let mut counter = 0;
        body.iter().for_each(|ast| match ast {
            Ast::WhileNode { body, .. } | Ast::IfNode { body, .. } => stack.push(body),
            Ast::RetNode(_) => counter += 1,
            _ => (),
        });
        abort_if_not!(counter == 1, "Error: function {} returns multiple values or returns nothing", name);
        stack.into_iter().for_each(|block| {
            let count = block.iter().filter(|ast| matches!(ast, Ast::RetNode(_))).count();
            abort_if_not!(count <= 1, "Error: function {} returns multiple values", name);
        })
    }
}