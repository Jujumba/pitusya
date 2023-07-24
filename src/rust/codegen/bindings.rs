use libc::c_void;

pub type LLVMPointer = *mut c_void;

extern "C" {
    pub fn PITUSYAPreInit();
    pub fn PITUSYAPostDestroy();
    pub fn PITUSYARunPasses();
    pub fn PITUSYASetParam(function: LLVMPointer, argn: *const i8, n: usize) -> LLVMPointer;
    pub fn PITUSYACountArgs(function: LLVMPointer) -> usize;
    pub fn PITUSYACheckFunction(function: LLVMPointer);
    pub fn PITUSYADeref(v: LLVMPointer, name: *const i8) -> LLVMPointer;
    #[allow(dead_code)]
    pub fn PITUSYADeclareFunction(name: *const i8, argc: usize) -> LLVMPointer;
    pub fn PITUSYACreateFunction(name: *const i8, argc: usize) -> LLVMPointer;
    pub fn PITUSYACreateVar(value: LLVMPointer, name: *const i8) -> LLVMPointer;
    pub fn PITUSYAAssignToVar(val: LLVMPointer, var: LLVMPointer);
    pub fn PITUSYAJITMain() -> i32;
    pub fn PITUSYABuildRet(v: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYAGenerateFP(n: f64) -> LLVMPointer;
    pub fn PITUSYACallFunction(callee: LLVMPointer, argc: usize, args: *mut LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildAdd(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildMul(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildSub(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildDiv(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub fn PITUSYABuildCmp(lhs: LLVMPointer, rhs: LLVMPointer, op: i32) -> LLVMPointer;
}
