use libc::c_void;

pub(super) type LLVMPointer = *mut c_void;

extern "C" {
    pub fn PITUSYAPreInit();
    pub fn PITUSYAPostDestroy();
    pub fn PITUSYACreateFunction(name: *const i8, argv: *const *const i8, argc: usize) -> LLVMPointer;
    pub fn PITUSYACreateVar(value: LLVMPointer, name: *const i8) -> LLVMPointer;
    pub fn PITUSYABuildRet(v: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYAGenerateFP(n: f64) -> LLVMPointer;
    pub fn PITUSYABuildAdd(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildMul(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildSub(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildDiv(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
}