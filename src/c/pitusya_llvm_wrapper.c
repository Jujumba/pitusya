#include <llvm-c/Types.h>
#include <llvm-c/Target.h>
#include "llvm-c/TargetMachine.h"
#include <llvm-c/Analysis.h>
#include "llvm-c/Core.h"
#include "llvm-c/Transforms/PassBuilder.h"
#include <malloc.h>
#include <string.h>
#include <stdbool.h>

LLVMContextRef CONTEXT = NULL;
LLVMModuleRef MODULE = NULL;
LLVMBuilderRef BUILDER = NULL;
LLVMTargetRef TARGET = NULL;
LLVMTargetMachineRef TM = NULL;
LLVMPassBuilderOptionsRef PB = NULL;

static void PITUSYAInitTarget(void) {
    LLVMInitializeNativeTarget();
    TARGET = LLVMGetFirstTarget();
    TM = LLVMCreateTargetMachine(TARGET, LLVMGetDefaultTargetTriple(), NULL, NULL, LLVMCodeGenLevelAggressive, LLVMRelocDefault, LLVMCodeModelDefault);
}
void PITUSYAPreInit() {
    PITUSYAInitTarget();
    CONTEXT = LLVMContextCreate();
    MODULE = LLVMModuleCreateWithNameInContext("pitusya module", CONTEXT);
    BUILDER = LLVMCreateBuilderInContext(CONTEXT);
    PB = LLVMCreatePassBuilderOptions();
    LLVMPassBuilderOptionsSetVerifyEach(PB, true);
}
void PITUSYAPostDestroy() {
    LLVMRunPasses(MODULE, "sroa,early-cse,simplifycfg,reassociate,mem2reg,instsimplify,instcombine,dce", TM, PB);
    LLVMDisposePassBuilderOptions(PB);
    LLVMDisposeBuilder(BUILDER);
    LLVMDumpModule(MODULE);
    LLVMDisposeModule(MODULE);
    LLVMContextDispose(CONTEXT);
}
LLVMValueRef PITUSYACreateFunction(const char* name, size_t argc) {
    LLVMTypeRef args[argc];
    for (size_t i = 0; i < argc; ++i) {
        args[i] = LLVMDoubleTypeInContext(CONTEXT);
    }
    LLVMValueRef function = LLVMAddFunction(MODULE, name, LLVMFunctionType(LLVMDoubleTypeInContext(CONTEXT), args, argc, 0));
    LLVMBasicBlockRef entryBlock = LLVMAppendBasicBlockInContext(CONTEXT, function, "entry");
    LLVMPositionBuilderAtEnd(BUILDER, entryBlock);
    return function;
}
LLVMValueRef PITUSYASetParam(LLVMValueRef function, const char* argn, size_t n) {
    LLVMSetValueName2(LLVMGetParam(function, n), argn, strlen(argn));
    return LLVMGetParam(function, n);
}
size_t PITUSYACountArgs(LLVMValueRef function) {
    return LLVMCountParams(function);
}
void PITUSYACheckFunction(LLVMValueRef function) {
    LLVMVerifyFunction(function, LLVMAbortProcessAction);
}
void PITUSYAAssignToVar(LLVMValueRef val, LLVMValueRef var) {
    LLVMBuildStore(BUILDER, val, var);
}
LLVMValueRef PITUSYACreateVar(LLVMValueRef val, const char* name) {
    LLVMValueRef var = LLVMBuildAlloca(BUILDER, LLVMDoubleTypeInContext(CONTEXT), name);
    PITUSYAAssignToVar(val, var);
    return var;
}
LLVMValueRef PITUSYADeref(LLVMValueRef v, const char* name) {
    return LLVMBuildLoad2(BUILDER, LLVMDoubleTypeInContext(CONTEXT), v, name);
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
LLVMValueRef PITUSYACallFunction(LLVMValueRef callee, size_t argc, LLVMValueRef *args) {
    return LLVMBuildCall2(BUILDER, LLVMGlobalGetValueType(callee), callee, args, argc, "calltmp");
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
LLVMValueRef PITUSYABuildCmp(LLVMValueRef lhs, LLVMValueRef rhs, int op) {
    return LLVMBuildFCmp(BUILDER, op, lhs, rhs, "cmptmp");
}