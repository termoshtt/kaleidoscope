#![allow(dead_code)]

use llvm::*;
use std::fmt::Debug;

#[derive(Debug)]
pub struct CodeGenError<'a> {
    pub comment: String,
    pub trace: Vec<&'a Debug>,
}

pub trait Ast: Debug {
    type Output;
    fn codegen<'a>(
        &'a self,
        &mut Module,
        &mut IRBuilder,
        &mut SymbolTable,
    ) -> Result<'a, Self::Output>;
}

impl<'a> CodeGenError<'a> {
    fn new(comment: &str, ast: &'a Debug) -> Self {
        CodeGenError {
            comment: comment.into(),
            trace: vec![ast],
        }
    }

    fn pushed<'b, 'c>(self, ast: &'b Debug) -> CodeGenError<'c>
    where
        'a: 'c,
        'b: 'c,
    {
        let mut trace: Vec<&'c Debug> = self.trace;
        trace.push(ast);
        CodeGenError {
            comment: self.comment,
            trace,
        }
    }
}

pub type Result<'a, T> = ::std::result::Result<T, CodeGenError<'a>>;

pub trait ExprAst: Ast<Output = ValueRef> {}
pub type Expr = Box<ExprAst<Output = ValueRef> + 'static>;

#[derive(Debug, Clone, PartialEq, new)]
pub struct Variable {
    name: String,
}

impl ExprAst for Variable {}
impl Ast for Variable {
    type Output = ValueRef;
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

impl ExprAst for Number {}
impl Ast for Number {
    type Output = ValueRef;
    fn codegen(&self, _: &mut Module, _: &mut IRBuilder, _: &mut SymbolTable) -> Result<ValueRef> {
        Ok(const_f64(self.value))
    }
}

#[derive(Debug, new)]
pub struct Call {
    callee: String,
    args: Vec<Expr>,
}

impl ExprAst for Call {}
impl Ast for Call {
    type Output = ValueRef;
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
    lhs: Expr,
    rhs: Expr,
}

impl ExprAst for Binary {}
impl Ast for Binary {
    type Output = ValueRef;
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

impl Ast for Proto {
    type Output = FunctionRef;
    fn codegen(
        &self,
        m: &mut Module,
        _: &mut IRBuilder,
        _: &mut SymbolTable,
    ) -> Result<FunctionRef> {
        let args = vec![f64_type(); self.args.len()];
        let f_ty = fn_type(f64_type(), &args);
        let f = m.create_function(&self.name, f_ty);
        f.get_args()
            .iter_mut()
            .zip(self.args.iter())
            .for_each(|(a, name)| a.set_name(name));
        Ok(f)
    }
}

impl Proto {
    fn annonymus() -> Self {
        Self::new("__anon_expr".into(), Vec::new())
    }
}

#[derive(Debug, new)]
pub struct Func {
    proto: Proto,
    body: Expr,
}

impl Ast for Func {
    type Output = FunctionRef;
    fn codegen(
        &self,
        m: &mut Module,
        ir: &mut IRBuilder,
        st: &mut SymbolTable,
    ) -> Result<FunctionRef> {
        let f = match m.get_function(&self.proto.name) {
            Some(f) => f,
            None => self.proto.codegen(m, ir, st).map_err(|e| e.pushed(self))?,
        };
        let bb = BasicBlock::new(f, "entry");
        ir.set_position(&bb);
        st.clear();
        for arg in f.get_args() {
            st.insert(arg.get_name(), arg);
        }
        let body = self.body.codegen(m, ir, st)?;
        ir.build_return(body);
        f.verify()
            .ok_or(CodeGenError::new("Function verification failed", self))?;
        Ok(f)
    }
}

impl Func {
    pub fn top_level_expr(expr: Expr) -> Self {
        Self::new(Proto::annonymus(), expr)
    }
}

#[derive(Debug, new)]
pub struct Extern {
    proto: Proto,
}

impl Ast for Extern {
    type Output = FunctionRef;
    fn codegen(
        &self,
        m: &mut Module,
        ir: &mut IRBuilder,
        st: &mut SymbolTable,
    ) -> Result<FunctionRef> {
        self.proto.codegen(m, ir, st)
    }
}
