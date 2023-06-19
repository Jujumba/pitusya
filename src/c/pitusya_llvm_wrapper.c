#include <llvm-c/Types.h>
#include <stdio.h>
#include <string.h>
#include <llvm-c/Target.h>
#include "llvm-c/Core.h"

typedef LLVMValueRef(*LLVMFunction)(LLVMBuilderRef, LLVMValueRef,LLVMValueRef, const char*);

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
static LLVMValueRef PITUSYAGetAnonExpr(LLVMFunction p, LLVMValueRef lhs, LLVMValueRef rhs, const char* name) {
    LLVMValueRef anon = LLVMAddFunction(MODULE, "__anon_expr", LLVMFunctionType(LLVMDoubleTypeInContext(CONTEXT), NULL, 0, 0));
    LLVMBasicBlockRef entryBlock = LLVMAppendBasicBlockInContext(CONTEXT, anon, "entry");
    LLVMPositionBuilderAtEnd(BUILDER, entryBlock);
    LLVMBuildRet(BUILDER, p(BUILDER, lhs, rhs, name));
    return anon;
}
LLVMValueRef PITUSYAGenerateFP(double n) {
    return LLVMConstReal(LLVMDoubleTypeInContext(CONTEXT), n);
}
LLVMValueRef PITUSYAGenerateString(char* s) {
    return LLVMConstString(s, strlen(s), 0);
}
LLVMValueRef PITUSYABuildAdd(LLVMValueRef lhs, LLVMValueRef rhs) {
    return PITUSYAGetAnonExpr(LLVMBuildFAdd, lhs, rhs, "addtmp");
}
LLVMValueRef PITUSYABuildMul(LLVMValueRef lhs, LLVMValueRef rhs) {
    return PITUSYAGetAnonExpr(LLVMBuildFMul, lhs, rhs, "multmp");
}
LLVMValueRef PITUSYABuildSub(LLVMValueRef lhs, LLVMValueRef rhs) {
    return PITUSYAGetAnonExpr(LLVMBuildSub, lhs, rhs, "subtmp");
}
LLVMValueRef PITUSYABuildDiv(LLVMValueRef lhs, LLVMValueRef rhs) {
    return PITUSYAGetAnonExpr(LLVMBuildFDiv, lhs, rhs, "divtmp");
}
void PITUSYAPrintIR(LLVMValueRef ir) {
    char* s = LLVMPrintValueToString(ir);
    printf("%s\n",s);
    LLVMDisposeMessage(s);
}
