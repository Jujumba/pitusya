mod bindings;

use std::cell::RefCell;
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
    vtable: RefCell<HashMap<String, LLVMPointer>>,
}

impl Cg {
    pub fn codegen(&self, ast: Ast) {
        match ast {
            Ast::FunctionNode { proto, body } => {
                let function = unsafe {
                    let function_name = cstr!(proto.name);
                    PITUSYACreateFunction(function_name.as_ptr(), proto.args.len())
                };

                let mut named_values = HashMap::new();
                Self::set_arguments(function, proto.args, &mut named_values);

                body.into_iter().for_each(|ast| {
                    Self::generate_ir(ast, &mut named_values);
                });

                unsafe { PITUSYACheckFunction(function) }
                self.vtable.borrow_mut().insert(proto.name, function);
            }
            Ast::EOF => (),
            _ => {
                abort!("Please report how you have bypassed the parser");
            }
        }
    }
    fn set_arguments(function: LLVMPointer, args: Vec<Ast>, placeholder: &mut HashMap<String, LLVMPointer>) {
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
    fn generate_ir(ast: Ast, named_values: &mut HashMap<String, LLVMPointer>) -> LLVMPointer {
        match ast {
            Ast::ValueNode(literal) => match literal {
                LiteralKind::Num(n) => unsafe { PITUSYAGenerateFP(n) },
                _ => todo!("Strings?"),
            },
            Ast::IdentifierNode(ident) => {
                match named_values.get(&ident) {
                    Some(var) => *var,
                    _ => {
                        abort!(format!("No variable {ident}. Consider creating it"));
                    }
                }
            }
            Ast::LetNode { assignee, value } => {
                let assignee_cname = cstr!(assignee);
                let var = unsafe {
                    let var = PITUSYACreateVar(Self::generate_ir(*value, named_values), assignee_cname.as_ptr());
                    PITUSYALoadVariable(var, assignee_cname.as_ptr())
                };
                named_values.insert(assignee, var);
                var
            }
            Ast::BinaryNode { left, right, op } => {
                let lhs = Self::generate_ir(*left, named_values);
                let rhs = Self::generate_ir(*right, named_values);
                match op {
                    BinaryOperatorKind::Addition => unsafe { PITUSYABuildAdd(lhs, rhs) },
                    BinaryOperatorKind::Multiplication => unsafe { PITUSYABuildMul(lhs, rhs) },
                    BinaryOperatorKind::Subtraction => unsafe { PITUSYABuildSub(lhs, rhs) },
                    BinaryOperatorKind::Division => unsafe { PITUSYABuildDiv(lhs, rhs) },
                    _ => todo!(),
                }
            }
            Ast::RetNode(ret) => unsafe { PITUSYABuildRet(Self::generate_ir(*ret, named_values)) },
            Ast::UnitNode(unit) => Self::generate_ir(*unit, named_values),
            _ => todo!(),
        }
    }
}
impl Drop for Cg {
    fn drop(&mut self) {
        unsafe { PITUSYAPostDestroy() };
    }
}
impl Default for Cg {
    fn default() -> Self {
        unsafe { PITUSYAPreInit() };
        Self {
            vtable: RefCell::new(HashMap::new()),
        }
    }
}
