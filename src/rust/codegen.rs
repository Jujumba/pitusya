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
    pub fn PITUSYAPrintIR(ir: LLVMPointer);
    pub fn PITUSYAWrapInFunction(v: LLVMPointer, block: *const i8) -> LLVMPointer;
    fn PITUSYAGenerateFP(n: f64) -> LLVMPointer;
    fn PITUSYABuildAdd(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    fn PITUSYABuildMul(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    fn PITUSYABuildSub(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    fn PITUSYABuildDiv(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
}

pub fn codegen(ast: Ast) -> LLVMPointer {
    let vtable = get_vtable();
    match ast {
        Ast::LetNode { assignee, value } => {
            let block = CString::new(assignee.as_str()).unwrap();
            unsafe {
                let value = PITUSYAWrapInFunction(generate_ir(*value), block.as_ptr());
                vtable.insert(assignee, value);
                value
            }
        }
        _ => unsafe { PITUSYAWrapInFunction(generate_ir(ast), "__anon_expr\0".as_ptr() as *const i8) }
    }
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
