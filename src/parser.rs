use ast;
use combine::char::spaces;
use combine::*;
use token::*;

pub fn number_expr<I: Stream<Item = char>>(
) -> impl Parser<Input = I, Output = Box<ast::Expr + 'static>> {
    number().map(|t| Box::new(ast::Number::new(t.as_number().unwrap())) as _)
}

pub fn var_expr<I: Stream<Item = char>>(
) -> impl Parser<Input = I, Output = Box<ast::Expr + 'static>> {
    ident().map(|t| Box::new(ast::Variable::new(t.as_ident().unwrap())) as _)
}

pub fn call_expr<I: Stream<Item = char>>(
) -> impl Parser<Input = I, Output = Box<ast::Expr + 'static>> {
    ident().then(|name| {
        let name = name.as_ident().unwrap();
        between(token('('), token(')'), sep_by(expr(), token(',')))
            .map(move |args| Box::new(ast::Call::new(name.clone(), args)) as _)
    })
}

pub fn expr<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Box<ast::Expr + 'static>> {
    between(spaces(), spaces(), number_expr().or(var_expr()))
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
