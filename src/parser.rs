use ast;
use combine::*;
use token::*;

pub fn number_expr<I: Stream<Item = char>>(
) -> impl Parser<Input = I, Output = Box<ast::Expr + 'static>> {
    number().map(|t| match t {
        Token::Number(f) => Box::new(ast::Number::new(f)) as _,
        _ => unreachable!(""),
    })
}

pub fn proto<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = ast::Proto> {
    ident().then(|name| {
        let name = name.as_ident().unwrap();
        between(
            token('('),
            token(')'),
            sep_by(ident().map(|arg| arg.as_ident().unwrap()), token(',')),
        ).map(move |args| ast::Proto::new(name.clone(), args))
    })
}
