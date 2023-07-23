use crate::ast::Ast;
use crate::abort;

pub struct PitusyaPassManager;
impl PitusyaPassManager {
    pub fn pipeline(&self, asts: &[Ast]) {
        for ast in asts {
            if let Ast::FunctionNode { proto, body} = ast {
                self.terminated(&proto.name, body);
            }
        }
    }
    fn terminated(&self, name: &str, body: &[Ast]) {
        let mut stack = vec![body];
        while let Some(body) = stack.pop() {
            let mut count: usize = 0;
            for ast in body {
                match ast {
                    Ast::IfNode { body, .. } => stack.push(body),
                    Ast::WhileNode { body, .. } => stack.push(body),
                    Ast::RetNode(_) => count += 1,
                    _ => ()
                }
            }
            if count == 0 {
                abort!(format!("Error: function `{name}` does not return a value!"));
            } else if count == 2 {
                abort!(format!("Error: function `{name}` returns multiple values!"));
            }
        }
    }
}