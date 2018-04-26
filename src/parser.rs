use ast;
use combine::char::spaces;
use combine::*;
use std::marker::PhantomData;
use token::*;

pub struct CallExpr<I>(PhantomData<fn(I) -> I>);
impl<I: Stream<Item = char>> Parser for CallExpr<I> {
    type Input = I;
    type Output = Box<ast::Expr>;
    #[inline]
    fn parse_stream(&mut self, input: I) -> ParseResult<Self::Output, Self::Input> {
        let mut p = ident()
            .and(between(token('('), token(')'), sep_by(expr(), token(','))))
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
    type Output = Box<ast::Expr>;
    #[inline]
    fn parse_stream(&mut self, input: I) -> ParseResult<Self::Output, Self::Input> {
        let num = number().map(|t| Box::new(ast::Number::new(t.as_number().unwrap())) as _);
        let var = ident().map(|t| Box::new(ast::Variable::new(t.as_ident().unwrap())) as _);
        let mut p = between(spaces(), spaces(), try(call_expr()).or(num).or(var));
        p.parse_stream(input)
    }
}
pub fn expr<I: Stream<Item = char>>() -> Expr<I> {
    Expr(PhantomData)
}

/// parse `f(a, b, c)`
pub fn proto<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Proto> {
    ident().then(|name| {
        let name = name.as_ident().unwrap();
        between(
            token('('),
            token(')'),
            sep_by(
                between(
                    spaces(),
                    spaces(),
                    ident().map(|arg| arg.as_ident().unwrap()),
                ),
                token(','),
            ),
        ).map(move |args| ast::Proto::new(name.clone(), args))
    })
}
