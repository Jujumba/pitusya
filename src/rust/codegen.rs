mod bindings;
mod var;

use std::collections::HashMap;
use std::ffi::CString;

use bindings::*;
use var::Variable;

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
    fn generate_ir(&self, ast: Ast, named_values: &mut HashMap<String, Variable>) -> LLVMPointer {
        match ast {
            Ast::ValueNode(literal) => match literal {
                LiteralKind::Num(n) => unsafe { PITUSYAGenerateFP(n) },
                _ => abort!("Strings are not impelemented yet!"),
            },
            Ast::IdentifierNode(ident) => match named_values.get(&ident) {
                Some(var) => var.value,
                _ => abort!(format!("No variable {ident}. Consider creating it")),
            },
            Ast::LetNode { assignee, value } => {
                let name = cstr!(assignee);
                let value = self.generate_ir(*value, named_values);
                let value = unsafe { PITUSYACreateVar(value, name.as_ptr()) };
                named_values.insert(assignee, Variable::new(value, false));
                value
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
                    .for_each(|ast| args.push(self.deref_or_generate(ast, named_values)));

                unsafe { PITUSYACallFunction(function, argc, args.as_mut_ptr()) }
            }
            Ast::BinaryNode { left, right, op } => match op {
                BinaryOperatorKind::Addition => unsafe {
                    PITUSYABuildAdd(
                        self.deref_or_generate(*left, named_values),
                        self.deref_or_generate(*right, named_values),
                    )
                },
                BinaryOperatorKind::Multiplication => unsafe {
                    PITUSYABuildMul(
                        self.deref_or_generate(*left, named_values),
                        self.deref_or_generate(*right, named_values),
                    )
                },
                BinaryOperatorKind::Subtraction => unsafe {
                    PITUSYABuildSub(
                        self.deref_or_generate(*left, named_values),
                        self.deref_or_generate(*right, named_values),
                    )
                },
                BinaryOperatorKind::Division => unsafe {
                    PITUSYABuildDiv(
                        self.deref_or_generate(*left, named_values),
                        self.deref_or_generate(*right, named_values),
                    )
                },
                BinaryOperatorKind::Comparision(cmp) => unsafe {
                    PITUSYABuildCmp(
                        self.deref_or_generate(*left, named_values),
                        self.deref_or_generate(*right, named_values),
                        cmp.into(),
                    )
                },
                BinaryOperatorKind::Assigment => unsafe {
                    let lhs = self.generate_ir(*left, named_values);
                    let rhs = self.generate_ir(*right, named_values);
                    PITUSYAAssignToVar(rhs, lhs);
                    rhs
                },
            },
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
            _ => abort!("Your code uses a not implemented yet feature. Thus aborting. Sorry"),
        }
    }
    fn set_arguments(&self, function: LLVMPointer, args: Vec<Ast>, placeholder: &mut HashMap<String, Variable>) {
        for (i, arg) in args.into_iter().enumerate() {
            if let Ast::IdentifierNode(arg) = arg {
                let param = unsafe {
                    let arg = cstr!(arg);
                    PITUSYASetParam(function, arg.as_ptr(), i)
                };
                placeholder.insert(arg, Variable::new(param, true));
            }
        }
    }
    fn create_function(&mut self, proto: Proto, body: Vec<Ast>) {
        let function = unsafe {
            let function_name = cstr!(proto.name);
            PITUSYACreateFunction(function_name.as_ptr(), proto.args.len())
        };

        let mut named_values = HashMap::<String, Variable>::new();
        self.set_arguments(function, proto.args, &mut named_values);

        body.into_iter().for_each(|ast| {
            self.generate_ir(ast, &mut named_values);
        });

        unsafe {
            PITUSYACheckFunction(function);
        }
        self.vtable.insert(proto.name, function);
    }
    fn deref_or_generate(&self, ast: Ast, named_values: &mut HashMap<String, Variable>) -> LLVMPointer {
        // FUCK YES FINALLY IT WORKS BUT IT IS SO BAAAAAAAAAAAAAAAAAAD
        if let Ast::IdentifierNode(ref ident) = ast {
            if let Some(var) = named_values.get(ident) {
                if !var.is_function_arg {
                    let ir = self.generate_ir(ast, named_values);
                    return unsafe { PITUSYADeref(ir, "deref\0".as_ptr() as *const i8) };
                } 
            }
        }
        self.generate_ir(ast, named_values)
        
    }
}
impl Drop for Cg {
    fn drop(&mut self) {
        if !self.contains_main {
            abort!("No main function. Consider creating it.");
        }
        unsafe { PITUSYAPostDestroy() };
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
