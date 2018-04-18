use super::Token;
use combine::char::digit;
use combine::*;
use std::marker::PhantomData;

pub struct Number<I>(PhantomData<fn(I) -> I>);

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

pub fn number<I>() -> Number<I>
where
    I: Stream<Item = char>,
{
    Number(PhantomData)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let mut num = number();
        let (num, remain) = num.parse("1.234").unwrap();
        assert_eq!(num, Token::Number(1.234));
        assert_eq!(remain, "");
    }
}
