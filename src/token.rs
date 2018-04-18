use combine::char::{alpha_num, digit, letter, spaces};
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
        many::<String, _>(alpha_num()).map(move |s| {
            let s = format!("{}{}", d, s);
            match s.as_str() {
                "def" => Token::Def,
                "extern" => Token::Extern,
                _ => Token::Identifier(s),
            }
        })
    })
}

pub fn parser<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    spaces().then(|_| eof().map(|_| Token::EOF).or(number().or(identifier())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_identifier() {
        let mut p = identifier();
        let (id, remain) = p.parse("a").unwrap();
        assert_eq!(id, Token::Identifier("a".to_string()));
        assert_eq!(remain, "");

        let (d, remain) = p.parse("def").unwrap();
        assert_eq!(d, Token::Def);
        assert_eq!(remain, "");

        let (e, remain) = p.parse("extern").unwrap();
        assert_eq!(e, Token::Extern);
        assert_eq!(remain, "");
    }

    #[test]
    fn parse_number() {
        let mut num = number();
        let (num, remain) = num.parse("1.234").unwrap();
        assert_eq!(num, Token::Number(1.234));
        assert_eq!(remain, "");
    }

    #[test]
    fn parse() {
        let mut p = parser();
        let (token, remain) = p.parse("def a 1.234").unwrap();
        assert_eq!(token, Token::Def);
        let (token, remain) = p.parse(remain).unwrap();
        assert_eq!(token, Token::Identifier("a".into()));
        let (token, remain) = p.parse(remain).unwrap();
        assert_eq!(token, Token::Number(1.234));
        let (token, remain) = p.parse(remain).unwrap();
        assert_eq!(token, Token::EOF);
        assert_eq!(remain, "");
    }
}
