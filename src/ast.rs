#![allow(dead_code)]

use llvm::*;
use std::fmt::Debug;

pub struct CodeGenError<'a> {
    comment: String,
    trace: Vec<&'a Ast>,
}

impl<'a> CodeGenError<'a> {
    fn pushed<'b, 'c>(self, ast: &'b Ast) -> CodeGenError<'c>
    where
        'a: 'c,
        'b: 'c,
    {
        let mut trace: Vec<&'c Ast> = self.trace;
        trace.push(ast);
        CodeGenError {
            comment: self.comment,
            trace,
        }
    }
}

pub type RValue<'a> = Result<Value, CodeGenError<'a>>;

pub trait Ast: Debug {
    fn codegen<'a>(&'a self, &mut Module, &mut IRBuilder, &mut SymbolTable) -> RValue<'a>;
}

pub trait Expr: Ast {}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Variable {
    name: String,
}

impl Expr for Variable {}

impl Ast for Variable {
    fn codegen(&self, _: &mut Module, _: &mut IRBuilder, table: &mut SymbolTable) -> RValue {
        match table.get(&self.name) {
            Some(&value) => Ok(value),
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
    fn codegen(&self, _: &mut Module, _: &mut IRBuilder, _: &mut SymbolTable) -> RValue {
        Ok(const_f64(self.value))
    }
}

#[derive(Debug, new)]
pub struct Call {
    callee: String,
    args: Vec<Box<Expr>>,
}

impl Expr for Call {}

impl Ast for Call {
    fn codegen(&self, _: &mut Module, _: &mut IRBuilder, _: &mut SymbolTable) -> RValue {
        unimplemented!()
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
    fn codegen(&self, m: &mut Module, ir: &mut IRBuilder, st: &mut SymbolTable) -> RValue {
        let lhs = self.lhs.codegen(m, ir, st).map_err(|e| e.pushed(self))?;
        let rhs = self.rhs.codegen(m, ir, st).map_err(|e| e.pushed(self))?;
        match self.op {
            Op::Add => Ok(ir.create_fadd(lhs, rhs, "addtmp")),
            Op::Sub => Ok(ir.create_fsub(lhs, rhs, "subtmp")),
            Op::Mul => Ok(ir.create_fmul(lhs, rhs, "multmp")),
            Op::Div => Ok(ir.create_fdiv(lhs, rhs, "divtmp")),
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
