#![allow(dead_code)]

use llvm_sys::core::*;
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::Debug;

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

pub struct CodeGenError<'a> {
    comment: String,
    trace: Vec<&'a Ast>,
}

pub type RValue<'a> = Result<LLVMValueRef, CodeGenError<'a>>;

pub trait Ast: Debug {
    fn codegen<'a>(&'a self, &mut Context) -> RValue<'a>;
}

pub trait Expr: Ast {}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Variable {
    name: String,
}

impl Expr for Variable {}

impl Ast for Variable {
    fn codegen(&self, context: &mut Context) -> RValue {
        match context.symble_table.get(&self.name) {
            Some(value) => Ok(*value),
            None => Err(CodeGenError {
                comment: format!("Undefined variable: {}", self.name),
                trace: vec![self],
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Number {
    value: f64,
}

impl Expr for Number {}

impl Ast for Number {
    fn codegen(&self, _: &mut Context) -> RValue {
        Ok(unsafe { LLVMConstReal(LLVMDoubleType(), self.value) })
    }
}

#[derive(Debug, new)]
pub struct Call {
    callee: String,
    args: Vec<Box<Expr>>,
}

impl Expr for Call {}

impl Ast for Call {
    fn codegen(&self, _: &mut Context) -> RValue {
        Ok(::std::ptr::null_mut())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, new)]
pub struct Binary {
    op: Op,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
}

impl Expr for Binary {}

impl Ast for Binary {
    fn codegen(&self, c: &mut Context) -> RValue {
        let lhs = self.lhs.codegen(c)?;
        let rhs = self.rhs.codegen(c)?;
        match self.op {
            Op::Add => Ok(c.create_fadd(lhs, rhs, "addtmp")),
            Op::Sub => Ok(c.create_fsub(lhs, rhs, "subtmp")),
            Op::Mul => Ok(c.create_fmul(lhs, rhs, "multmp")),
            Op::Div => Ok(c.create_fdiv(lhs, rhs, "divtmp")),
        }
    }
}

#[derive(Debug, PartialEq, new)]
pub struct Proto {
    name: String,
    args: Vec<String>,
}

#[derive(Debug, new)]
pub struct Func {
    proto: Proto,
    body: Box<Expr>,
}

#[derive(Debug, new)]
pub struct Extern {
    proto: Proto,
}
