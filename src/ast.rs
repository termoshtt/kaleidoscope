#![allow(dead_code)]

use llvm_sys::core::{LLVMConstReal, LLVMDoubleType};
use llvm_sys::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;

pub type SymbolTable = HashMap<String, LLVMValueRef>;

pub struct CodeGenError<'a> {
    comment: String,
    trace: Vec<&'a Ast>,
}

pub type RValue<'a> = Result<LLVMValueRef, CodeGenError<'a>>;

pub trait Ast: Debug {
    fn codegen<'a>(&'a self, &mut SymbolTable) -> RValue<'a>;
}

pub trait Expr: Debug {}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Variable {
    name: String,
}

impl Expr for Variable {}

impl Ast for Variable {
    fn codegen(&self, table: &mut SymbolTable) -> RValue {
        match table.get(&self.name) {
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
    fn codegen(&self, _: &mut SymbolTable) -> RValue {
        Ok(unsafe { LLVMConstReal(LLVMDoubleType(), self.value) })
    }
}

#[derive(Debug, new)]
pub struct Call {
    callee: String,
    args: Vec<Box<Expr>>,
}

impl Expr for Call {}

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
