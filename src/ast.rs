#![allow(dead_code)]

use llvm::*;
use std::fmt::Debug;

pub struct CodeGenError<'a> {
    comment: String,
    trace: Vec<&'a Ast>,
}

/// Marker trait for code generator
pub trait Ast: Debug {}

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

pub type Result<'a, T> = ::std::result::Result<T, CodeGenError<'a>>;

pub trait Expr: Ast {
    fn codegen<'a>(&'a self, &mut Module, &mut IRBuilder, &mut SymbolTable)
        -> Result<'a, ValueRef>;
}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Variable {
    name: String,
}

impl Ast for Variable {}
impl Expr for Variable {
    fn codegen(
        &self,
        _: &mut Module,
        _: &mut IRBuilder,
        table: &mut SymbolTable,
    ) -> Result<ValueRef> {
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

impl Ast for Number {}
impl Expr for Number {
    fn codegen(&self, _: &mut Module, _: &mut IRBuilder, _: &mut SymbolTable) -> Result<ValueRef> {
        Ok(const_f64(self.value))
    }
}

#[derive(Debug, new)]
pub struct Call {
    callee: String,
    args: Vec<Box<Expr>>,
}

impl Ast for Call {}
impl Expr for Call {
    fn codegen(
        &self,
        m: &mut Module,
        ir: &mut IRBuilder,
        st: &mut SymbolTable,
    ) -> Result<ValueRef> {
        let f = m.get_function(&self.callee).ok_or(CodeGenError {
            comment: format!("Unknown function: {}", self.callee),
            trace: vec![self],
        })?;
        // TODO: #args check
        let args = self.args
            .iter()
            .map(|a| a.codegen(m, ir, st).map_err(|e| e.pushed(self)))
            .collect::<Result<Vec<_>>>()?;
        Ok(ir.build_call(f, &args, "calltmp"))
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

impl Ast for Binary {}
impl Expr for Binary {
    fn codegen(
        &self,
        m: &mut Module,
        ir: &mut IRBuilder,
        st: &mut SymbolTable,
    ) -> Result<ValueRef> {
        let lhs = self.lhs.codegen(m, ir, st).map_err(|e| e.pushed(self))?;
        let rhs = self.rhs.codegen(m, ir, st).map_err(|e| e.pushed(self))?;
        match self.op {
            Op::Add => Ok(ir.build_fadd(lhs, rhs, "addtmp")),
            Op::Sub => Ok(ir.build_fsub(lhs, rhs, "subtmp")),
            Op::Mul => Ok(ir.build_fmul(lhs, rhs, "multmp")),
            Op::Div => Ok(ir.build_fdiv(lhs, rhs, "divtmp")),
        }
    }
}

#[derive(Debug, PartialEq, new)]
pub struct Proto {
    name: String,
    args: Vec<String>,
}

impl Ast for Proto {}
impl Proto {
    fn codegen(
        &self,
        m: &mut Module,
        _: &mut IRBuilder,
        _: &mut SymbolTable,
    ) -> Result<FunctionRef> {
        let args = vec![f64_type(); self.args.len()];
        let f = fn_type(f64_type(), &args);
        // TODO: set name of args
        Ok(m.create_function(&self.name, f))
    }
}

#[derive(Debug, new)]
pub struct Func {
    proto: Proto,
    body: Box<Expr>,
}

impl Ast for Func {}
impl Func {
    fn codegen(
        &self,
        m: &mut Module,
        ir: &mut IRBuilder,
        st: &mut SymbolTable,
    ) -> Result<FunctionRef> {
        let proto = match m.get_function(&self.proto.name) {
            Some(f) => f,
            None => self.proto.codegen(m, ir, st).map_err(|e| e.pushed(self))?,
        };
        let bb = BasicBlock::new(proto, "entry");
        ir.set_position(&bb);
    }
}

#[derive(Debug, new)]
pub struct Extern {
    proto: Proto,
}
