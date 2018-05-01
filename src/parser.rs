use ast;
use combine::char::spaces;
use combine::*;
use std::marker::PhantomData;
use token;

#[derive(Debug)]
pub enum InputAst {
    Expr(ast::Expr),
    Func(ast::Func),
    Extern(ast::Extern),
}

pub fn input<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = InputAst> {
    extern_()
        .map(|e| InputAst::Extern(e))
        .or(func().map(|f| InputAst::Func(f)))
        .or(expr().map(|e| InputAst::Expr(e)))
}

fn op<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Op> {
    token('+')
        .or(token('-'))
        .or(token('*'))
        .or(token('/'))
        .map(|op| match op {
            '+' => ast::Op::Add,
            '-' => ast::Op::Sub,
            '*' => ast::Op::Mul,
            '/' => ast::Op::Div,
            _ => unreachable!(""),
        })
}

fn num<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Expr> {
    token::number().map(|t| Box::new(ast::Number::new(t.as_number().unwrap())) as _)
}

fn var<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Expr> {
    token::ident().map(|t| Box::new(ast::Variable::new(t.as_ident().unwrap())) as _)
}

fn unary_expr<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Expr> {
    paren(try(binary_expr()).or(try(call_expr())).or(num()).or(var()))
        .or(try(call_expr()).or(num()).or(var()))
}

pub struct BinaryExpr<I>(PhantomData<fn(I) -> I>);
impl<I: Stream<Item = char>> Parser for BinaryExpr<I> {
    type Input = I;
    type Output = ast::Expr;
    #[inline]
    fn parse_stream(&mut self, input: I) -> ParseResult<Self::Output, Self::Input> {
        let mut p = unary_expr()
            .and(spanned(op()))
            .and(expr())
            .map(|((lhs, op), rhs)| Box::new(ast::Binary::new(op, lhs, rhs)) as _);
        p.parse_stream(input)
    }
}
pub fn binary_expr<I: Stream<Item = char>>() -> BinaryExpr<I> {
    BinaryExpr(PhantomData)
}

pub struct CallExpr<I>(PhantomData<fn(I) -> I>);
impl<I: Stream<Item = char>> Parser for CallExpr<I> {
    type Input = I;
    type Output = ast::Expr;
    #[inline]
    fn parse_stream(&mut self, input: I) -> ParseResult<Self::Output, Self::Input> {
        let mut p = token::ident()
            .and(paren(sep_by(expr(), token(','))))
            .map(move |(name, args)| Box::new(ast::Call::new(name.as_ident().unwrap(), args)) as _);
        p.parse_stream(input)
    }
}
pub fn call_expr<I: Stream<Item = char>>() -> CallExpr<I> {
    CallExpr(PhantomData)
}

pub struct Expr<I>(PhantomData<fn(I) -> I>);
impl<I: Stream<Item = char>> Parser for Expr<I> {
    type Input = I;
    type Output = ast::Expr;
    #[inline]
    fn parse_stream(&mut self, input: I) -> ParseResult<Self::Output, Self::Input> {
        let mut p = spanned(try(binary_expr()).or(unary_expr()));
        p.parse_stream(input)
    }
}
pub fn expr<I: Stream<Item = char>>() -> Expr<I> {
    Expr(PhantomData)
}

/// parse `f(a, b, c)`
pub fn proto<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Proto> {
    token::ident().then(|name| {
        let name = name.as_ident().unwrap();
        let ident = spanned(token::ident().map(|arg| arg.as_ident().unwrap()));
        paren(sep_by(ident, token(','))).map(move |args| ast::Proto::new(name.clone(), args))
    })
}

pub fn func<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Func> {
    spaces()
        .and(token::def())
        .and(spanned(proto()))
        .and(expr())
        .map(|((_, p), body)| ast::Func::new(p, body))
}

pub fn extern_<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Extern> {
    spaces()
        .and(token::extern_())
        .and(spanned(proto()))
        .map(|(_, p)| ast::Extern::new(p))
}

fn spanned<I, O, P>(p: P) -> impl Parser<Input = I, Output = O>
where
    I: Stream<Item = char>,
    P: Parser<Input = I, Output = O>,
{
    between(spaces(), spaces(), p)
}

fn paren<I, O, P>(p: P) -> impl Parser<Input = I, Output = O>
where
    I: Stream<Item = char>,
    P: Parser<Input = I, Output = O>,
{
    between(token('('), token(')'), spanned(p))
}
