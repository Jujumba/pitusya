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
    LLVMDumpModule(MODULE);
    LLVMDisposeModule(MODULE);
    LLVMContextDispose(CONTEXT);
}
LLVMValueRef PITUSYACreateFunction(const char* name, const char** argn, size_t argc) {
    LLVMTypeRef args[argc];
    for (size_t i = 0; i < argc; ++i) {
        args[i] = LLVMDoubleTypeInContext(CONTEXT);
    }
    LLVMValueRef function = LLVMAddFunction(MODULE, name, LLVMFunctionType(LLVMDoubleTypeInContext(CONTEXT), args, argc, 0));
    for (size_t i = 0; i < argc; ++i) {
        LLVMSetValueName2(LLVMGetParam(function, i), argn[i], strlen(argn[i]));
    }
    LLVMBasicBlockRef entryBlock = LLVMAppendBasicBlockInContext(CONTEXT, function, "entry");
    LLVMPositionBuilderAtEnd(BUILDER, entryBlock);
    // LLVMVerifyFunction(function, LLVMAbortProcessAction); // todo: extract to separate function
    return function;
}
LLVMValueRef PITUSYACreateVar(LLVMValueRef value, const char* name) {
    LLVMValueRef var = LLVMBuildAlloca(BUILDER, LLVMDoubleTypeInContext(CONTEXT), name);
    LLVMBuildStore(BUILDER, value, var);
    return var;
}
LLVMValueRef PITUSYABuildRet(LLVMValueRef v) {
    return LLVMBuildRet(BUILDER, v);
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