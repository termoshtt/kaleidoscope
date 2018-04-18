extern crate combine;

use combine::char::digit;
use combine::*;
use std::marker::PhantomData;

#[derive(Debug)]
enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
}

struct Number<I>(PhantomData<fn(I) -> I>);

impl<I> Parser for Number<I>
where
    I: Stream<Item = char>,
{
    type Input = I;
    type Output = Token;

    #[inline]
    fn parse_stream(&mut self, input: I) -> ParseResult<Self::Output, Self::Input> {
        let mut parser = many1::<String, _>(digit().or(token('.'))).map(|c| {
            c.parse::<f64>()
                .map(|f| Token::Number(f))
                .expect("Cannot parse float")
        });
        parser.parse_stream(input)
    }
}

fn number<I>() -> Number<I>
where
    I: Stream<Item = char>,
{
    Number(PhantomData)
}

fn main() {
    let mut num = number();
    println!("{:?}", num.parse("1.234"));
    println!("{:?}", num.parse("1234"));
}
