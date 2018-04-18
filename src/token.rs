use combine::char::{alpha_num, digit, letter};
use combine::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
}

pub fn number<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    many1::<String, _>(digit().or(token('.'))).map(|c| {
        c.parse::<f64>()
            .map(|f| Token::Number(f))
            .expect("Cannot parse float")
    })
}

pub fn identifier<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    letter().then(|d| {
        many::<String, _>(alpha_num()).map(move |s| Token::Identifier(format!("{}{}", d, s)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_identifier() {
        let mut ident = identifier();
        let (id, remain) = ident.parse("a").unwrap();
        assert_eq!(id, Token::Identifier("a".to_string()));
        assert_eq!(remain, "");
    }

    #[test]
    fn parse_number() {
        let mut num = number();
        let (num, remain) = num.parse("1.234").unwrap();
        assert_eq!(num, Token::Number(1.234));
        assert_eq!(remain, "");
    }
}
