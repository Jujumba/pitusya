use libc::c_void;

pub(super) type LLVMPointer = *mut c_void;

extern "C" {
    pub(crate) fn PITUSYAPreInit();
    pub(crate) fn PITUSYAPostDestroy();
    pub(super) fn PITUSYACreateFunction(name: *const i8, argv: *const *const i8, argc: usize) -> LLVMPointer;
    pub(super) fn PITUSYACreateVar(value: LLVMPointer, name: *const i8) -> LLVMPointer;
    pub(super) fn PITUSYABuildRet(v: LLVMPointer) -> LLVMPointer;
    pub(super) fn PITUSYAGenerateFP(n: f64) -> LLVMPointer;
    pub(super) fn PITUSYABuildAdd(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub(super) fn PITUSYABuildMul(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub(super) fn PITUSYABuildSub(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
    pub(super) fn PITUSYABuildDiv(lhs: LLVMPointer, rhs: LLVMPointer) -> LLVMPointer;
}