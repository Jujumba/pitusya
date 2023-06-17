#include <llvm-c/Types.h>
#include <stdio.h>
#include <string.h>
#include <llvm-c/Target.h>
#include "llvm-c/Core.h"

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
    LLVMContextDispose(CONTEXT);
}
LLVMValueRef PITUSYAGenerateFP(double n) {
    return LLVMConstReal(LLVMDoubleTypeInContext(CONTEXT), n);
}
LLVMValueRef PITUSYAGenerateString(char* s) {
    return LLVMConstString(s, strlen(s), 0);
}
LLVMValueRef PITUSYABuildAdd(LLVMValueRef lhs, LLVMValueRef rhs) {
    return LLVMBuildAdd(BUILDER, lhs, rhs, "addtmp");
}
LLVMValueRef PITUSYABuildMul(LLVMValueRef lhs, LLVMValueRef rhs) {
    return LLVMBuildMul(BUILDER, lhs, rhs, "multpm");
}
LLVMValueRef PITUSYABuildSub(LLVMValueRef lhs, LLVMValueRef rhs) {
    return LLVMBuildSub(BUILDER, lhs, rhs, "subtmp");
}
LLVMValueRef PITUSYABuildDiv(LLVMValueRef lhs, LLVMValueRef rhs) {
    return LLVMBuildMul(BUILDER, lhs, rhs, "divtmp");
}
void PITUSYAPrintIR(LLVMValueRef ir) {
    char* s = LLVMPrintValueToString(ir);
    printf("%s\n",s);
    LLVMDisposeMessage(s);
}
