use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;

pub struct Context {
    builder: LLVMBuilderRef,
    symble_table: HashMap<String, LLVMValueRef>,
}

macro_rules! create_f { ($create:ident, $llvm_func:ident) => {
pub fn $create(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
    let name = CString::new(name).expect("Cannot cast to CString");
    unsafe { $llvm_func(self.builder, lhs, rhs, name.as_ptr()) }
}
}} // create_f

impl Context {
    pub fn new() -> Self {
        let builder = unsafe { LLVMCreateBuilder() };
        let symble_table = HashMap::new();
        Context {
            builder,
            symble_table,
        }
    }
    create_f!(create_fadd, LLVMBuildFAdd);
    create_f!(create_fsub, LLVMBuildFSub);
    create_f!(create_fmul, LLVMBuildFMul);
    create_f!(create_fdiv, LLVMBuildFDiv);
}
