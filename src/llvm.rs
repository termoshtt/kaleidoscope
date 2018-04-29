//! Minimal safe wrapper of llvm-sys for Kaleidscope compiler

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;

pub struct Context(LLVMContextRef);
pub struct IRBuilder(LLVMBuilderRef);
pub struct Module(LLVMModuleRef);

pub type ValueRef = LLVMValueRef;
pub type FunctionRef = LLVMValueRef;
pub type SymbolTable = HashMap<String, ValueRef>;

pub type TypeRef = LLVMTypeRef;

impl Context {
    pub fn new() -> Self {
        Context(unsafe { LLVMContextCreate() })
    }

    pub fn get_global() -> Self {
        Context(unsafe { LLVMGetGlobalContext() })
    }
}

macro_rules! build_binop { ($build:ident, $llvm_func:ident) => {
pub fn $build(&mut self, lhs: ValueRef, rhs: ValueRef, name: &str) -> ValueRef {
    let name = CString::new(name).expect("Cannot cast to CString");
    unsafe { $llvm_func(self.0, lhs, rhs, name.as_ptr()) }
}
}} // build_binop

impl IRBuilder {
    pub fn new() -> Self {
        IRBuilder(unsafe { LLVMCreateBuilder() })
    }

    build_binop!(build_fadd, LLVMBuildFAdd);
    build_binop!(build_fsub, LLVMBuildFSub);
    build_binop!(build_fmul, LLVMBuildFMul);
    build_binop!(build_fdiv, LLVMBuildFDiv);

    pub fn build_call(&mut self, func: FunctionRef, args: &[ValueRef], name: &str) -> ValueRef {
        let name = CString::new(name).expect("Cannot cast to CString");
        unsafe {
            LLVMBuildCall(
                self.0,
                func,
                args.as_ptr() as *mut _, // XXX: Is this real safe?
                args.len() as u32,
                name.as_ptr(),
            )
        }
    }
}

impl Module {
    pub fn get_function(&mut self, name: &str) -> Option<FunctionRef> {
        let name = CString::new(name).expect("Cannot cast to CString");
        let ptr = unsafe { LLVMGetNamedFunction(self.0, name.as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(ptr)
        }
    }

    pub fn create_function(&mut self, name: &str, ty: TypeRef) -> FunctionRef {
        let name = CString::new(name).expect("Cannot cast to CString");
        unsafe { LLVMAddFunction(self.0, name.as_ptr(), ty) }
    }
}

pub fn const_f64(value: f64) -> ValueRef {
    unsafe { LLVMConstReal(LLVMDoubleType(), value) }
}

pub fn fn_type(ret: TypeRef, params: &[TypeRef]) -> TypeRef {
    const FALSE: LLVMBool = 0;
    unsafe { LLVMFunctionType(ret, params.as_ptr() as *mut _, params.len() as u32, FALSE) }
}

pub fn f64_type() -> TypeRef {
    unsafe { LLVMDoubleType() }
}
