use std::cell::OnceCell;
use std::collections::HashMap;
use std::ffi::CString;

use libc::c_void;

use crate::ast::*;
use crate::lexer::tokens::*;

type LLVMPointer = *mut c_void;

static mut VTABLE: OnceCell<HashMap<String, LLVMPointer>> = OnceCell::new();

extern "C" {
    pub fn PITUSYAPreInit();
    pub fn PITUSYAPostDestroy();
    fn PITUSYACreateFunction(name: *const i8, argv: *const *const i8, argc: usize) -> LLVMPointer;
    fn PITUSYABuildRet(v: LLVMPointer) -> LLVMPointer;
    fn PITUSYAGenerateFP(n: f64) -> LLVMPointer;
    fn PITUSYABuildAdd(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    fn PITUSYABuildMul(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    fn PITUSYABuildSub(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    fn PITUSYABuildDiv(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
}

pub fn codegen(ast: Ast) {
    let vtable = get_vtable();
    match ast {
        Ast::FunctionNode { proto, body } => {
            if let Ast::PrototypeNode { name, args } = *proto {
                let function_name = CString::new(name.as_str()).unwrap();
                let args_pointers = fetch_arguments(args);
                let argv: Vec<*const i8> = args_pointers.iter().map(|arg| arg.as_ptr()).collect();
                unsafe {
                    let f = PITUSYACreateFunction(function_name.as_ptr(), argv.as_ptr(), argv.len());
                    body.into_iter().for_each(|i| {
                        generate_ir(i);
                    });
                    vtable.insert(name, f);
                }
            }
        }
        _ => todo!(),
    }
}
fn fetch_arguments(args: Vec<Ast>) -> Vec<CString> {
    let mut cstrings = Vec::new();
    for arg in args {
        if let Ast::IdentifierNode(arg) = arg {
            cstrings.push(CString::new(arg.as_str()).unwrap());
        }
    }
    cstrings
}
fn generate_ir(ast: Ast) -> LLVMPointer {
    match ast {
        Ast::ValueNode(literal) => match literal {
            LiteralKind::Num(n) => unsafe { PITUSYAGenerateFP(n) },
            _ => todo!("Strings?")
        },
        Ast::BinaryNode { left, right, op } => {
            let (lhs, rhs) = (generate_ir(*left), generate_ir(*right));
            match op {
                BinaryOperatorKind::Addition => unsafe { PITUSYABuildAdd(lhs, rhs) },
                BinaryOperatorKind::Multiplication => unsafe { PITUSYABuildMul(lhs, rhs) },
                BinaryOperatorKind::Subtraction => unsafe { PITUSYABuildSub(lhs, rhs) },
                BinaryOperatorKind::Division => unsafe { PITUSYABuildDiv(lhs, rhs) },
                _ => todo!()
            }
        }
        Ast::RetNode(ret) => unsafe { PITUSYABuildRet(generate_ir(*ret)) },
        Ast::UnitNode(unit) => generate_ir(*unit),
        _ => todo!()
    }
}
fn get_vtable() -> &'static mut HashMap<String, LLVMPointer> {
    unsafe {
        VTABLE.get_or_init(HashMap::new);
        VTABLE.get_mut().unwrap()
    }
}
