use super::bindings::*;

pub struct Variable {
    pub(super) value: LLVMValueRef,
    pub(super) is_function_arg: bool,
}
impl Variable {
    pub fn new(value: LLVMValueRef, is_function_arg: bool) -> Self {
        Self { value, is_function_arg }
    }
}
