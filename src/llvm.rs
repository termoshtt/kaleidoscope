//! Minimal safe wrapper of llvm-sys for Kaleidscope compiler

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::ptr::null_mut;

pub struct Context(LLVMContextRef);
pub struct IRBuilder(LLVMBuilderRef);
pub struct Module(LLVMModuleRef);

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
    ValueRef(unsafe { $llvm_func(self.0, lhs.0, rhs.0, name.as_cstring().as_ptr()) })
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
        ValueRef(unsafe {
            LLVMBuildCall(
                self.0,
                func.0,
                args.as_ptr() as *mut _, // XXX: Is this real safe?
                args.len() as u32,
                name.as_cstring().as_ptr(),
            )
        })
    }

    pub fn build_return(&mut self, ret: ValueRef) -> ValueRef {
        ValueRef(unsafe { LLVMBuildRet(self.0, ret.0) })
    }

    pub fn set_position(&mut self, bb: &BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.0, bb.0) };
    }
}

impl Module {
    pub fn new(name: &str) -> Self {
        Module(unsafe { LLVMModuleCreateWithName(name.as_cstring().as_ptr()) })
    }

    pub fn get_function(&mut self, name: &str) -> Option<FunctionRef> {
        let ptr = unsafe { LLVMGetNamedFunction(self.0, name.as_cstring().as_ptr()) };
        if ptr.is_null() {
            None
        } else {
            Some(FunctionRef(ptr))
        }
    }

    pub fn create_function(&mut self, name: &str, ty: TypeRef) -> FunctionRef {
        FunctionRef(unsafe { LLVMAddFunction(self.0, name.as_cstring().as_ptr(), ty) })
    }

    pub fn to_string(&self) -> String {
        unsafe {
            let ptr = LLVMPrintModuleToString(self.0);
            CStr::from_ptr(ptr)
                .to_str()
                .expect("Cannot convert into string")
                .into()
        }
    }
}

pub struct BasicBlock(LLVMBasicBlockRef);

impl BasicBlock {
    pub fn new(func: FunctionRef, name: &str) -> Self {
        BasicBlock(unsafe { LLVMAppendBasicBlock(func.0, name.as_cstring().as_ptr()) })
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FunctionRef(LLVMValueRef);

impl FunctionRef {
    pub fn num_args(&self) -> usize {
        unsafe { LLVMCountParams(self.0) as usize }
    }

    pub fn get_args(&self) -> Vec<ValueRef> {
        let n = self.num_args();
        let mut p = vec![null_mut(); n];
        unsafe { LLVMGetParams(self.0, p.as_mut_ptr()) };
        p.into_iter().map(|p| ValueRef(p)).collect()
    }

    pub fn verify(&self) -> Option<()> {
        use llvm_sys::analysis::*;
        let s = unsafe {
            LLVMVerifyFunction(self.0, LLVMVerifierFailureAction::LLVMPrintMessageAction)
        };
        if s == 0 {
            Some(())
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ValueRef(LLVMValueRef);

impl ValueRef {
    pub fn get_name(&self) -> String {
        unsafe { CStr::from_ptr(LLVMGetValueName(self.0)) }
            .to_str()
            .expect("Non utf8 name")
            .into()
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe { LLVMSetValueName(self.0, name.as_cstring().as_ptr()) };
    }
}

pub fn const_f64(value: f64) -> ValueRef {
    ValueRef(unsafe { LLVMConstReal(LLVMDoubleType(), value) })
}

pub fn fn_type(ret: TypeRef, params: &[TypeRef]) -> TypeRef {
    const FALSE: LLVMBool = 0;
    unsafe { LLVMFunctionType(ret, params.as_ptr() as *mut _, params.len() as u32, FALSE) }
}

pub fn f64_type() -> TypeRef {
    unsafe { LLVMDoubleType() }
}

trait AsCString {
    fn as_cstring(self) -> CString;
}

impl<'a> AsCString for &'a str {
    fn as_cstring(self) -> CString {
        CString::new(self).expect("Cannot cast to CString")
    }
}
