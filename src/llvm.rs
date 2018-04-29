use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;

pub struct Context(LLVMContextRef);
pub struct IRBuilder(LLVMBuilderRef);
pub struct Module(LLVMModuleRef);

pub type SymbolTable = HashMap<String, LLVMValueRef>;
pub type Value = LLVMValueRef;

impl Context {
    pub fn new() -> Self {
        Context(unsafe { LLVMContextCreate() })
    }

    pub fn get_global() -> Self {
        Context(unsafe { LLVMGetGlobalContext() })
    }
}

macro_rules! create_f { ($create:ident, $llvm_func:ident) => {
pub fn $create(&mut self, lhs: LLVMValueRef, rhs: LLVMValueRef, name: &str) -> LLVMValueRef {
    let name = CString::new(name).expect("Cannot cast to CString");
    unsafe { $llvm_func(self.0, lhs, rhs, name.as_ptr()) }
}
}} // create_f

impl IRBuilder {
    pub fn new() -> Self {
        IRBuilder(unsafe { LLVMCreateBuilder() })
    }

    create_f!(create_fadd, LLVMBuildFAdd);
    create_f!(create_fsub, LLVMBuildFSub);
    create_f!(create_fmul, LLVMBuildFMul);
    create_f!(create_fdiv, LLVMBuildFDiv);
}

pub fn const_f64(value: f64) -> LLVMValueRef {
    unsafe { LLVMConstReal(LLVMDoubleType(), value) }
}
