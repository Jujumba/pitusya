#include <llvm-c/Types.h>
#include <llvm-c/Target.h>
#include <llvm-c/Analysis.h>
#include "llvm-c/Core.h"
#include <stdio.h>
#include <string.h>

LLVMContextRef CONTEXT = NULL;
LLVMModuleRef MODULE = NULL;
LLVMBuilderRef BUILDER = NULL;

void PITUSYAPreInit() {
    LLVMInitializeAllTargets();
    CONTEXT = LLVMContextCreate();
    MODULE = LLVMModuleCreateWithNameInContext("pitusya module", CONTEXT);
    BUILDER = LLVMCreateBuilderInContext(CONTEXT);
}
void PITUSYAPostDestroy() {
    LLVMDisposeBuilder(BUILDER);
    LLVMDisposeModule(MODULE);
    LLVMContextDispose(CONTEXT);
}
LLVMValueRef PITUSYAWrapInAnonExpr(LLVMValueRef v) {
    LLVMValueRef anon = LLVMAddFunction(MODULE, "__anon_expr", LLVMFunctionType(LLVMDoubleTypeInContext(CONTEXT), NULL, 0, 0));
    LLVMBasicBlockRef entryBlock = LLVMAppendBasicBlockInContext(CONTEXT, anon, "entry");
    LLVMPositionBuilderAtEnd(BUILDER, entryBlock);
    LLVMBuildRet(BUILDER, v);
    LLVMVerifyFunction(anon, LLVMAbortProcessAction);
    return anon;
}
LLVMValueRef PITUSYAGenerateFP(double n) {
    return LLVMConstReal(LLVMDoubleTypeInContext(CONTEXT), n);
}
LLVMValueRef PITUSYAGenerateString(char* s, size_t len) {
    return LLVMConstString(s, len, 0);
}
LLVMValueRef PITUSYABuildAdd(LLVMValueRef lhs, LLVMValueRef rhs) {
    return LLVMBuildFAdd(BUILDER, lhs, rhs, "addtmp");
}
LLVMValueRef PITUSYABuildMul(LLVMValueRef lhs, LLVMValueRef rhs) {
    return LLVMBuildFMul(BUILDER, lhs, rhs, "multmp");
}
LLVMValueRef PITUSYABuildSub(LLVMValueRef lhs, LLVMValueRef rhs) {
    return LLVMBuildSub(BUILDER, lhs, rhs, "subtmp");
}
LLVMValueRef PITUSYABuildDiv(LLVMValueRef lhs, LLVMValueRef rhs) {
    return LLVMBuildFDiv(BUILDER, lhs, rhs, "divtmp");
}
void PITUSYAPrintIR(LLVMValueRef ir) {
    char* s = LLVMPrintValueToString(ir);
    printf("%s\n",s);
    LLVMDisposeMessage(s);
}