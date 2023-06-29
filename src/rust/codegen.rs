mod bindings;

use bindings::*;

use std::cell::OnceCell;
use std::collections::HashMap;
use std::ffi::CString;

use crate::ast::*;
use crate::lexer::tokens::*;

static mut VTABLE: OnceCell<HashMap<String, LLVMPointer>> = OnceCell::new();

pub struct Codegenerator;

impl Codegenerator {
    pub fn codegen(&self, ast: Ast) {
        let vtable = self.get_vtable();
        match ast {
            Ast::FunctionNode { proto, body } => {
                if let Ast::PrototypeNode { name, args } = *proto {
                    let function_name = CString::new(name.as_str()).unwrap();
                    let args_pointers = self.fetch_arguments(args);
                    let argv: Vec<*const i8> = args_pointers.iter().map(|arg| arg.as_ptr()).collect();
                    unsafe {
                        let f = PITUSYACreateFunction(function_name.as_ptr(), argv.as_ptr(), argv.len());
                        body.into_iter().for_each(|i| {
                            self.generate_ir(i);
                        });
                        vtable.insert(name, f);
                    }
                }
            }
            _ => todo!(),
        }
    }
    fn fetch_arguments(&self, args: Vec<Ast>) -> Vec<CString> {
        let mut cstrings = Vec::new();
        for arg in args {
            if let Ast::IdentifierNode(arg) = arg {
                cstrings.push(CString::new(arg.as_str()).unwrap());
            }
        }
        cstrings
    }
    fn generate_ir(&self, ast: Ast) -> LLVMPointer {
        match ast {
            Ast::ValueNode(literal) => match literal {
                LiteralKind::Num(n) => unsafe { PITUSYAGenerateFP(n) },
                _ => todo!("Strings?"),
            },
            Ast::LetNode { assignee, value } => {
                let assignee_cname = CString::new(assignee.as_str()).unwrap(); // todo
                unsafe { PITUSYACreateVar(self.generate_ir(*value), assignee_cname.as_ptr()) }
            }
            Ast::BinaryNode { left, right, op } => {
                let (lhs, rhs) = (self.generate_ir(*left), self.generate_ir(*right));
                match op {
                    BinaryOperatorKind::Addition => unsafe { PITUSYABuildAdd(lhs, rhs) },
                    BinaryOperatorKind::Multiplication => unsafe { PITUSYABuildMul(lhs, rhs) },
                    BinaryOperatorKind::Subtraction => unsafe { PITUSYABuildSub(lhs, rhs) },
                    BinaryOperatorKind::Division => unsafe { PITUSYABuildDiv(lhs, rhs) },
                    _ => todo!(),
                }
            }
            Ast::RetNode(ret) => unsafe { PITUSYABuildRet(self.generate_ir(*ret)) },
            Ast::UnitNode(unit) => self.generate_ir(*unit),
            _ => todo!(),
        }
    }
    fn get_vtable(&self) -> &'static mut HashMap<String, LLVMPointer> {
        unsafe {
            VTABLE.get_or_init(HashMap::new);
            VTABLE.get_mut().unwrap()
        }
    }
}
impl Drop for Codegenerator {
    fn drop(&mut self) {
        unsafe { PITUSYAPostDestroy() };
    }
}
impl Default for Codegenerator {
    fn default() -> Self {
        unsafe { PITUSYAPreInit() };
        Self {}
    }
}
