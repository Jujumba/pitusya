#![allow(clippy::wildcard_imports)]
#![allow(clippy::cast_possible_truncation)]
pub use llvm_sys::prelude::*;

use std::ffi::CStr;
use std::ffi::CString;

use llvm_sys::analysis::LLVMVerifierFailureAction;
use llvm_sys::analysis::LLVMVerifyFunction;
use llvm_sys::core::*;
use llvm_sys::error::LLVMConsumeError;
use llvm_sys::error::LLVMGetErrorMessage;
use llvm_sys::execution_engine::LLVMLinkInMCJIT;
use llvm_sys::orc2::lljit::*;
use llvm_sys::orc2::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::LLVMCreateTargetMachine;
use llvm_sys::target_machine::LLVMGetDefaultTargetTriple;
use llvm_sys::target_machine::LLVMGetFirstTarget;
use llvm_sys::target_machine::LLVMTargetMachineRef;
use llvm_sys::target_machine::LLVMTargetRef;
use llvm_sys::target_machine::{LLVMCodeGenOptLevel, LLVMCodeModel, LLVMRelocMode};
use llvm_sys::transforms::pass_builder::*;
use llvm_sys::LLVMRealPredicate;

use crate::abort;
use crate::lexer::tokens::ComparisionOpKind;

pub struct LLVMWrapper {
    context: LLVMContextRef,
    module: LLVMModuleRef,
    builder: LLVMBuilderRef,
    target: LLVMTargetRef,
    target_machine: LLVMTargetMachineRef,
    pass_builder: LLVMPassBuilderOptionsRef,
    execution_sesion: LLVMOrcExecutionSessionRef,
    jd: LLVMOrcJITDylibRef,
    jit: LLVMOrcLLJITRef,
}
#[allow(clippy::unused_self)]
impl LLVMWrapper {
    fn null() -> Self {
        Self {
            context: std::ptr::null_mut(),
            module: std::ptr::null_mut(),
            builder: std::ptr::null_mut(),
            target: std::ptr::null_mut(),
            target_machine: std::ptr::null_mut(),
            pass_builder: std::ptr::null_mut(),
            execution_sesion: std::ptr::null_mut(),
            jd: std::ptr::null_mut(),
            jit: std::ptr::null_mut(),
        }
    }
    unsafe fn init_target(&mut self) {
        LLVM_InitializeNativeTarget();
        LLVM_InitializeNativeAsmPrinter();
        LLVM_InitializeNativeAsmParser();
        self.target = LLVMGetFirstTarget();
        self.target_machine = LLVMCreateTargetMachine(
            self.target,
            LLVMGetDefaultTargetTriple(),
            std::ptr::null(),
            std::ptr::null(),
            LLVMCodeGenOptLevel::LLVMCodeGenLevelAggressive,
            LLVMRelocMode::LLVMRelocDefault,
            LLVMCodeModel::LLVMCodeModelJITDefault,
        );
    }
    pub unsafe fn new() -> Self {
        let mut wrapper = Self::null();
        wrapper.init_target();
        wrapper.context = LLVMContextCreate();
        wrapper.module = LLVMModuleCreateWithNameInContext("a town with an ocean view\0".as_ptr().cast(), wrapper.context);
        wrapper.builder = LLVMCreateBuilderInContext(wrapper.context);
        wrapper.pass_builder = LLVMCreatePassBuilderOptions();
        wrapper.create_jit();
        LLVMPassBuilderOptionsSetVerifyEach(wrapper.pass_builder, 1);
        LLVMSetDataLayout(wrapper.module, LLVMOrcLLJITGetDataLayoutStr(wrapper.jit));
        wrapper
    }
    pub unsafe fn jit_main(&mut self) -> i32 {
        let resource_tracker = LLVMOrcJITDylibGetDefaultResourceTracker(self.jd);
        let thc = LLVMOrcCreateNewThreadSafeContext();
        let tsm = LLVMOrcCreateNewThreadSafeModule(self.module, thc);
        LLVMOrcLLJITAddLLVMIRModule(self.jit, self.jd, tsm);

        // Horrible...
        let mut address: LLVMOrcExecutorAddress = 0;
        LLVMOrcLLJITLookup(self.jit, std::ptr::addr_of_mut!(address), "main\0".as_ptr().cast());
        let p: fn() -> f64 = std::mem::transmute(address);
        let res = p() as i32;

        LLVMOrcDisposeThreadSafeContext(thc);
        LLVMOrcResourceTrackerRemove(resource_tracker);
        res
    }
    pub unsafe fn create_jit(&mut self) {
        let err = LLVMOrcCreateLLJIT(std::ptr::addr_of_mut!(self.jit), LLVMOrcCreateLLJITBuilder());
        if !err.is_null() {
            let msg = LLVMGetErrorMessage(err);
            abort!("{}", CStr::from_ptr(msg).to_str().unwrap()); // Todo: string is not deallocated
        }
        LLVMConsumeError(err);
        self.execution_sesion = LLVMOrcLLJITGetExecutionSession(self.jit);
        self.jd = LLVMOrcLLJITGetMainJITDylib(self.jit);
        LLVMLinkInMCJIT();
        
        self.link_with_process();
        self.link_with_runtime();
    }
    pub unsafe fn run_passes(&self) {
        LLVMRunPasses(
            self.module,
            "sroa,early-cse,simplifycfg,reassociate,mem2reg,instsimplify,instcombine,dce\0"
                .as_ptr()
                .cast(),
            self.target_machine,
            self.pass_builder,
        );
    }
    pub unsafe fn declare_function(&self, name: &str, argc: usize) -> LLVMValueRef {
        let name = CString::new(name).unwrap();
        let mut arguments = Vec::with_capacity(argc);
        for _ in 0..argc {
            arguments.push(LLVMDoubleTypeInContext(self.context));
        }
        LLVMAddFunction(
            self.module,
            name.as_ptr(),
            LLVMFunctionType(LLVMDoubleTypeInContext(self.context), arguments.as_mut_ptr(), argc as u32, 0),
        )
    }
    pub unsafe fn create_function(&self, name: &str, argc: usize) -> LLVMValueRef {
        let function = self.declare_function(name, argc);
        let entry = LLVMAppendBasicBlockInContext(self.context, function, "entry\0".as_ptr().cast()); // Todo: bad casts
        LLVMPositionBuilderAtEnd(self.builder, entry);
        function
    }
    pub unsafe fn set_param2function(&self, function: LLVMValueRef, argn: &str, index: usize) -> LLVMValueRef {
        let argn = CString::new(argn).unwrap();
        let param = LLVMGetParam(function, index as _);
        LLVMSetValueName2(param, argn.as_ptr(), argn.as_bytes().len());
        param
    }
    #[inline]
    unsafe fn get_current_function(&self) -> LLVMValueRef {
        LLVMGetBasicBlockParent(LLVMGetInsertBlock(self.builder))
    }
    pub unsafe fn create_condition(&self, cond: LLVMValueRef) -> LLVMBasicBlockRef {
        let function = self.get_current_function();

        let then = LLVMAppendBasicBlockInContext(self.context, function, "then\0".as_ptr().cast());
        let merge = LLVMAppendBasicBlockInContext(self.context, function, "merge\0".as_ptr().cast());
        LLVMBuildCondBr(
            self.builder,
            self.i1cmp(cond, self.gen_fp(0.0), ComparisionOpKind::NeEq),
            then,
            merge,
        );

        LLVMPositionBuilderAtEnd(self.builder, then);
        merge
    }
    pub unsafe fn terminate_condition(&self, merge: LLVMBasicBlockRef, ret: bool) {
        if !ret {
            LLVMBuildBr(self.builder, merge);
        }
        LLVMPositionBuilderAtEnd(self.builder, merge);
    }
    pub unsafe fn create_loop(&self) -> (LLVMBasicBlockRef, LLVMBasicBlockRef) {
        let function = self.get_current_function();
        let loop_body = LLVMAppendBasicBlockInContext(self.context, function, "loop\0".as_ptr().cast());
        let merge = LLVMAppendBasicBlockInContext(self.context, function, "merge\0".as_ptr().cast());
        LLVMBuildBr(self.builder, loop_body);

        LLVMPositionBuilderAtEnd(self.builder, loop_body);

        (loop_body, merge)
    }
    pub unsafe fn terminate_loop(&self, cond: LLVMValueRef, loop_body: LLVMBasicBlockRef, merge: LLVMBasicBlockRef, ret: bool) {
        if !ret {
            LLVMBuildCondBr(self.builder, self.i1cmp(cond, self.gen_fp(0.0), ComparisionOpKind::NeEq), loop_body, merge);
        }
        LLVMPositionBuilderAtEnd(self.builder, merge);
    }
    pub unsafe fn count_args(&self, function: LLVMValueRef) -> usize {
        LLVMCountParams(function) as _
    }
    pub unsafe fn check_function(&self, function: LLVMValueRef) {
        LLVMVerifyFunction(function, LLVMVerifierFailureAction::LLVMAbortProcessAction);
    }
    pub unsafe fn call_function(&self, callee: LLVMValueRef, argc: usize, arguments: *mut LLVMValueRef) -> LLVMValueRef {
        LLVMBuildCall2(
            self.builder,
            LLVMGlobalGetValueType(callee),
            callee,
            arguments,
            argc as _,
            "calltmp\0".as_ptr().cast(),
        )
    }
    pub unsafe fn create_var(&self, value: LLVMValueRef, name: &str) -> LLVMValueRef {
        let name = CString::new(name).unwrap();
        let var = LLVMBuildAlloca(self.builder, LLVMDoubleTypeInContext(self.context), name.as_ptr());
        self.assign2var(value, var);
        var
    }
    pub unsafe fn assign2var(&self, var: LLVMValueRef, value: LLVMValueRef) {
        LLVMBuildStore(self.builder, var, value);
    }
    pub unsafe fn deref(&self, v: LLVMValueRef, name: &str) -> LLVMValueRef {
        let name = CString::new(name).unwrap();
        LLVMBuildLoad2(self.builder, LLVMDoubleTypeInContext(self.context), v, name.as_ptr())
    }
    pub unsafe fn ret(&self, v: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildRet(self.builder, v)
    }
    pub unsafe fn gen_fp(&self, n: f64) -> LLVMValueRef {
        LLVMConstReal(LLVMDoubleTypeInContext(self.context), n)
    }
    #[allow(dead_code)]
    pub unsafe fn gen_string(&self, s: &str) -> LLVMValueRef {
        let s = CString::new(s).unwrap();
        LLVMConstString(s.as_ptr(), s.as_bytes().len() as _, 0)
    }
    pub unsafe fn add(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildFAdd(self.builder, lhs, rhs, "addtmp\0".as_ptr().cast())
    }
    pub unsafe fn mul(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildFMul(self.builder, lhs, rhs, "multmp\0".as_ptr().cast())
    }
    pub unsafe fn sub(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildFSub(self.builder, lhs, rhs, "subtmp\0".as_ptr().cast())
    }
    pub unsafe fn div(&self, lhs: LLVMValueRef, rhs: LLVMValueRef) -> LLVMValueRef {
        LLVMBuildFDiv(self.builder, lhs, rhs, "divtmp\0".as_ptr().cast())
    }
    pub unsafe fn cmp(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, op: ComparisionOpKind) -> LLVMValueRef {
        let cmp = self.i1cmp(lhs, rhs, op);
        LLVMBuildSIToFP(
            self.builder,
            cmp,
            LLVMDoubleTypeInContext(self.context),
            "casttmp\0".as_ptr().cast(),
        )
    }
    unsafe fn i1cmp(&self, lhs: LLVMValueRef, rhs: LLVMValueRef, op: ComparisionOpKind) -> LLVMValueRef {
        LLVMBuildFCmp(self.builder, op.into(), lhs, rhs, "cmptmp\0".as_ptr().cast())
    }
    fn global_prefix(&self) -> i8{
        unsafe { LLVMOrcLLJITGetGlobalPrefix(self.jit) }
    }
    unsafe fn link_with_process(&self) {
        let mut proc_syms_gen: LLVMOrcDefinitionGeneratorRef = std::ptr::null_mut();
        let err = LLVMOrcCreateDynamicLibrarySearchGeneratorForProcess(
            &mut proc_syms_gen as _,
            self.global_prefix(),
            None,
            std::ptr::null_mut(),
        );
        if !err.is_null() {
            let msg = LLVMGetErrorMessage(err);
            libc::printf("%s\0".as_ptr().cast(), msg);
            abort!("Link error!");
        }
        LLVMOrcJITDylibAddGenerator(self.jd, proc_syms_gen);
    }
    unsafe fn link_with_runtime(&self) {
        let mut proc_syms_gen: LLVMOrcDefinitionGeneratorRef = std::ptr::null_mut();
        let pitusya_std = CString::new(if cfg!(windows) {
            "pitusyastd.dll"
        } else {
            "libpitusyastd.so"
        }).unwrap();
        let err = LLVMOrcCreateDynamicLibrarySearchGeneratorForPath(
            std::ptr::addr_of_mut!(proc_syms_gen),
            pitusya_std.as_ptr(),
            self.global_prefix(),
            None,
            std::ptr::null_mut()
        );
        if !err.is_null() {
            let msg = LLVMGetErrorMessage(err);
            libc::printf("%s\0".as_ptr().cast(), msg);
            abort!("Link error!");
        }
        LLVMOrcJITDylibAddGenerator(self.jd, proc_syms_gen);
    }
}
impl Drop for LLVMWrapper {
    fn drop(&mut self) {
        unsafe {
            LLVMOrcDisposeLLJIT(self.jit);
            LLVMDisposePassBuilderOptions(self.pass_builder);
            LLVMDisposeBuilder(self.builder);
            LLVMContextDispose(self.context);
        }
    }
}
#[allow(clippy::from_over_into)]
impl Into<LLVMRealPredicate> for ComparisionOpKind {
    fn into(self) -> LLVMRealPredicate {
        match self {
            Self::Equals => LLVMRealPredicate::LLVMRealOEQ,
            Self::Bigger => LLVMRealPredicate::LLVMRealOGT,
            Self::BiggerOrEq => LLVMRealPredicate::LLVMRealOGE,
            Self::Less => LLVMRealPredicate::LLVMRealOLT,
            Self::LessOrEq => LLVMRealPredicate::LLVMRealOLE,
            Self::NeEq => LLVMRealPredicate::LLVMRealONE,
        }
    }
}
