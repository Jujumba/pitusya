use super::bindings::LLVMPointer;

pub struct Variable {
    pub(super) value: LLVMPointer,
    pub(super) is_function_arg: bool,
}
impl Variable {
    pub fn new(value: LLVMPointer, is_function_arg: bool) -> Self {
        Self { value, is_function_arg }
    }
}
