mod bindings;
mod var;

use std::collections::HashMap;

use bindings::*;
use var::Variable;

use crate::abort;
use crate::ast::*;
use crate::lexer::tokens::{BinaryOperatorKind, LiteralKind};

pub struct Cg {
    vtable: HashMap<String, LLVMValueRef>,
    wrapper: LLVMWrapper,
    contains_main: bool,
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
            Ast::ExternNode(_) => abort!("Extern definitons are not implemented yet. Sorry"),
            Ast::EOF => (),
            _ => abort!("Please report how you have bypassed the parser"),
        }
    }
    fn generate_ir(&mut self, ast: Ast, named_values: &mut HashMap<String, Variable>) -> LLVMValueRef {
        match ast {
            Ast::ValueNode(literal) => {
                let LiteralKind::Num(n) = literal else {
                    abort!("Strings are not impelemented yet!")
                };
                unsafe { self.wrapper.gen_fp(n) }
            }
            Ast::IdentifierNode(ident) => {
                if let Some(var) = named_values.get(&ident) {
                    var.value
                } else {
                    abort!(format!("No variable {ident}. Consider creating it"))
                }
            }
            Ast::LetNode { assignee, value } => {
                let value = self.generate_ir(*value, named_values);
                let value = unsafe { self.wrapper.create_var(value, &assignee) };
                named_values.insert(assignee, Variable::new(value, false));
                value
            }
            Ast::CallNode(proto) => {
                let function = if let Some(f) = self.vtable.get(&proto.name) {
                    *f
                } else {
                    abort!(format!("No function {}. Define it before calling", proto.name))
                };

                let argc = unsafe { self.wrapper.count_args(function) };
                if argc != proto.args.len() {
                    abort!(format!(
                        "Incorrect number of arguments passed to {}. Expected {}, but got {}",
                        proto.name,
                        argc,
                        proto.args.len()
                    ));
                }

                let mut args = Vec::with_capacity(argc);
                proto
                    .args
                    .into_iter()
                    .for_each(|ast| args.push(self.deref_or_generate(ast, named_values)));

                unsafe { self.wrapper.call_function(function, argc, args.as_mut_ptr()) }
            }
            Ast::BinaryNode { left, right, op } => match op {
                BinaryOperatorKind::Addition => unsafe {
                    let lhs = self.deref_or_generate(*left, named_values);
                    let rhs = self.deref_or_generate(*right, named_values);
                    self.wrapper.add(lhs, rhs)
                },
                BinaryOperatorKind::Multiplication => unsafe {
                    let lhs = self.deref_or_generate(*left, named_values);
                    let rhs = self.deref_or_generate(*right, named_values);
                    self.wrapper.mul(lhs, rhs)
                },
                BinaryOperatorKind::Subtraction => unsafe {
                    let lhs = self.deref_or_generate(*left, named_values);
                    let rhs = self.deref_or_generate(*right, named_values);
                    self.wrapper.sub(lhs, rhs)
                },
                BinaryOperatorKind::Division => unsafe {
                    let lhs = self.deref_or_generate(*left, named_values);
                    let rhs = self.deref_or_generate(*right, named_values);
                    self.wrapper.div(lhs, rhs)
                },
                BinaryOperatorKind::Comparision(cmp) => unsafe {
                    let lhs = self.deref_or_generate(*left, named_values);
                    let rhs = self.deref_or_generate(*right, named_values);
                    self.wrapper.cmp(lhs, rhs, cmp.into())
                },
                BinaryOperatorKind::Assigment => unsafe {
                    if let Ast::IdentifierNode(ref ident) = *left {
                        if named_values.get(ident).unwrap().is_function_arg {
                            abort!(format!("Cannot assign to const variable {ident}"));
                        }
                    }
                    let lhs = self.generate_ir(*left, named_values);
                    let rhs = self.generate_ir(*right, named_values);
                    self.wrapper.assign2var(rhs, lhs);
                    rhs
                },
            },
            Ast::RetNode(ret) => unsafe {
                let ret = self.deref_or_generate(*ret, named_values);
                self.wrapper.ret(ret)
            },
            Ast::UnitNode(unit) => self.generate_ir(*unit, named_values),
            _ => abort!("Your code uses a not implemented yet feature. Thus aborting. Sorry"),
        }
    }
    fn set_arguments(&mut self, function: LLVMValueRef, args: Vec<Ast>, placeholder: &mut HashMap<String, Variable>) {
        for (i, arg) in args.into_iter().enumerate() {
            if let Ast::IdentifierNode(arg) = arg {
                let param = unsafe {
                    self.wrapper.set_param2function(function, &arg, i)
                };
                placeholder.insert(arg, Variable::new(param, true));
            }
        }
    }
    fn create_function(&mut self, proto: Proto, body: Vec<Ast>) {
        let function = unsafe {
            self.wrapper.create_function(&proto.name, proto.args.len())
        };

        let mut named_values = HashMap::<String, Variable>::new();
        self.set_arguments(function, proto.args, &mut named_values);

        self.vtable.insert(proto.name, function);

        for instruction in body {
            self.generate_ir(instruction, &mut named_values);
        }

        unsafe {
            self.wrapper.check_function(function);
        }
    }
    fn deref_or_generate(&mut self, ast: Ast, named_values: &mut HashMap<String, Variable>) -> LLVMValueRef {
        // FUCK YES FINALLY IT WORKS BUT IT IS SO BAAAAAAAAAAAAAAAAAAD
        if let Ast::IdentifierNode(ref ident) = ast {
            if let Some(var) = named_values.get(ident) {
                if !var.is_function_arg {
                    let ir = self.generate_ir(ast, named_values);
                    return unsafe { self.wrapper.deref(ir, "deref") };
                }
            }
        }
        self.generate_ir(ast, named_values)
    }
    pub fn exec(mut self) -> i32 {
        if !self.contains_main {
            abort!("No main function. Consider creating it.");
        }
        unsafe {
            self.wrapper.run_passes();
            self.wrapper.jit_main()
        }
    }
}
impl Default for Cg {
    fn default() -> Self {
        Self {
            vtable: HashMap::new(),
            wrapper: unsafe { LLVMWrapper::new() },
            contains_main: false,
        }
    }
}
