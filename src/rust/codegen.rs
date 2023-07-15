mod bindings;

use std::collections::HashMap;
use std::ffi::CString;

use bindings::*;

use crate::abort;
use crate::ast::*;
use crate::lexer::tokens::*;

macro_rules! cstr {
    ($string: expr) => {
        CString::new($string.as_str()).unwrap()
    };
}

pub struct Cg {
    vtable: HashMap<String, LLVMPointer>,
    contains_main: bool
}

impl Cg {
    pub fn codegen(&mut self, ast: Ast) {
        match ast {
            Ast::FunctionNode { proto, body } => {
                if &proto.name == "main" {
                    self.contains_main = true;
                }
                if self.vtable.contains_key(&proto.name) {
                    abort!(format!("Function {} already exists!", proto.name));
                }
                self.create_function(proto, body);
            }
            Ast::EOF => (),
            _ => abort!("Please report how you have bypassed the parser"),
        }
    }
    fn generate_ir(&self, ast: Ast, named_values: &mut HashMap<String, LLVMPointer>) -> LLVMPointer {
        match ast {
            Ast::ValueNode(literal) => match literal {
                LiteralKind::Num(n) => unsafe { PITUSYAGenerateFP(n) },
                _ => abort!("Strings are not impelemented yet!"),
            },
            Ast::IdentifierNode(ident) => match named_values.get(&ident) {
                Some(var) => *var,
                _ => abort!(format!("No variable {ident}. Consider creating it")),
            },
            Ast::LetNode { assignee, value } => {
                let name = cstr!(assignee);
                let value = self.generate_ir(*value, named_values);
                let var = unsafe { PITUSYACreateVar(value, name.as_ptr()) };
                named_values.insert(assignee, var);
                var
            }
            Ast::CallNode(proto) => {
                let function = match self.vtable.get(&proto.name) {
                    Some(f) => *f,
                    _ => abort!(format!("No function {}. Define it before calling", proto.name)),
                };

                let argc = unsafe { PITUSYACountArgs(function) };
                if argc != proto.args.len() {
                    abort!(format!(
                        "Incorrect number of arguments passed to {}. Expected {}, but got {}",
                        proto.name,
                        argc,
                        proto.args.len()
                    ))
                }

                let mut args = Vec::new();
                proto
                    .args
                    .into_iter()
                    .for_each(|ast| args.push(self.generate_ir(ast, named_values)));

                unsafe { PITUSYACallFunction(function, argc, args.as_mut_ptr()) }
            }
            Ast::BinaryNode { left, right, op } => {
                // Some awful Rust. AS IS :)
                let is_lhs_ident = matches!(*left, Ast::IdentifierNode(_));
                let lhs = self.generate_ir(*left, named_values);
                let lhs_derefed = if is_lhs_ident{
                    unsafe { PITUSYADeref(lhs, "deref\0".as_ptr() as *const i8) }
                } else {
                    lhs
                };

                let is_rhs_ident = matches!(*right, Ast::IdentifierNode(_));
                let rhs = self.generate_ir(*right, named_values);
                let rhs_derefed = if is_rhs_ident {
                    unsafe { PITUSYADeref(lhs, "deref\0".as_ptr() as *const i8) }
                } else {
                    rhs
                };

                match op {
                    BinaryOperatorKind::Addition => unsafe { PITUSYABuildAdd(lhs_derefed, rhs_derefed) },
                    BinaryOperatorKind::Multiplication => unsafe { PITUSYABuildMul(lhs_derefed, rhs_derefed) },
                    BinaryOperatorKind::Subtraction => unsafe { PITUSYABuildSub(lhs_derefed, rhs_derefed) },
                    BinaryOperatorKind::Division => unsafe { PITUSYABuildDiv(lhs_derefed, rhs_derefed) },
                    BinaryOperatorKind::Comparision(cmp) => unsafe { PITUSYABuildCmp(lhs_derefed, rhs_derefed, cmp.into()) }
                    BinaryOperatorKind::Assigment => unsafe {
                        PITUSYAAssignToVar(rhs, lhs);
                        lhs
                    },
                }
            }
            Ast::RetNode(ret) => {
                // Once again. Really bad Rust code.
                let is_ident = matches!(*ret, Ast::IdentifierNode(_));
                let mut ret = self.generate_ir(*ret, named_values);
                if is_ident {
                    ret = unsafe { PITUSYADeref(ret, "deref\0".as_ptr() as *const i8) }
                }
                unsafe { PITUSYABuildRet(ret) }
            }
            Ast::UnitNode(unit) => self.generate_ir(*unit, named_values),
            _ => todo!(),
        }
    }
    fn set_arguments(&self, function: LLVMPointer, args: Vec<Ast>, placeholder: &mut HashMap<String, LLVMPointer>) {
        for (i, arg) in args.into_iter().enumerate() {
            if let Ast::IdentifierNode(arg) = arg {
                let param = unsafe {
                    let arg = cstr!(arg);
                    PITUSYASetParam(function, arg.as_ptr(), i)
                };
                placeholder.insert(arg, param);
            }
        }
    }
    fn create_function(&mut self, proto: Proto, body: Vec<Ast>) {
            let function = unsafe {
                let function_name = cstr!(proto.name);
                PITUSYACreateFunction(function_name.as_ptr(), proto.args.len())
            };

            let mut named_values = HashMap::new();
            self.set_arguments(function, proto.args, &mut named_values);

            body.into_iter().for_each(|ast| {
                self.generate_ir(ast, &mut named_values);
            });

            unsafe { PITUSYACheckFunction(function); }
            self.vtable.insert(proto.name, function);
    }
}
impl Drop for Cg {
    fn drop(&mut self) {
        unsafe { PITUSYAPostDestroy() };
        if !self.contains_main {
            abort!("No main function. Consider creating it.");
        }
    }
}
impl Default for Cg {
    fn default() -> Self {
        unsafe { PITUSYAPreInit() };
        Self {
            vtable: HashMap::new(),
            contains_main: false
        }
    }
}
