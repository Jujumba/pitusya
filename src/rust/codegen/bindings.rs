use libc::c_void;

pub type LLVMPointer = *mut c_void;

extern "C" {
    pub fn PITUSYAPreInit();
    pub fn PITUSYAPostDestroy();
    pub fn PITUSYASetParam(function: LLVMPointer, argn: *const i8, n: usize) -> LLVMPointer;
    pub fn PITUSYACheckFunction(function: LLVMPointer);
    pub fn PITUSYALoadVariable(v: LLVMPointer, name: *const i8) -> LLVMPointer;
    pub fn PITUSYACreateFunction(name: *const i8, argc: usize) -> LLVMPointer;
    pub fn PITUSYACreateVar(value: LLVMPointer, name: *const i8) -> LLVMPointer;
    pub fn PITUSYABuildRet(v: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYAGenerateFP(n: f64) -> LLVMPointer;
    pub fn PITUSYABuildAdd(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildMul(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildSub(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildDiv(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
}
