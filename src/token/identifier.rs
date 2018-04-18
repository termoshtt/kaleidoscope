use super::Token;
use combine::char::{alpha_num, letter};
use combine::*;
use std::marker::PhantomData;

/// Parse for identifier
pub struct Identifier<I>(PhantomData<fn(I) -> I>);

impl<I> Parser for Identifier<I>
where
    I: Stream<Item = char>,
{
    type Input = I;
    type Output = Token;

    #[inline]
    fn parse_stream(&mut self, input: I) -> ParseResult<Self::Output, Self::Input> {
        let mut parser = letter().then(|d| {
            many::<String, _>(alpha_num()).map(move |s| Token::Identifier(format!("{}{}", d, s)))
        });
        parser.parse_stream(input)
    }
}

pub fn identifier<I>() -> Identifier<I>
where
    I: Stream<Item = char>,
{
    Identifier(PhantomData)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let mut ident = identifier();
        let (id, remain) = ident.parse("a").unwrap();
        assert_eq!(id, Token::Identifier("a".to_string()));
        assert_eq!(remain, "");
    }
}
