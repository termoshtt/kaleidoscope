#![allow(dead_code)]
use std::fmt::Debug;

pub trait Expr: Debug {}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Variable {
    name: String,
}

impl Expr for Variable {}

#[derive(Debug, Clone, PartialEq, new)]
pub struct Number {
    value: f64,
}

impl Expr for Number {}

#[derive(Debug, new)]
pub struct Call {
    callee: String,
    args: Vec<Box<Expr>>,
}

impl Expr for Call {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Op {
    Add,
    Mul,
    Neg,
    Div,
}

#[derive(Debug, new)]
pub struct Binary {
    op: Op,
    rhs: Box<Expr>,
    lhs: Box<Expr>,
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
