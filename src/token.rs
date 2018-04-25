use combine::char::{alpha_num, digit, letter, spaces, string};
use combine::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
}

impl Token {
    pub fn as_number(self) -> Option<f64> {
        match self {
            Token::Number(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_ident(self) -> Option<String> {
        match self {
            Token::Identifier(f) => Some(f),
            _ => None,
        }
    }
}

pub fn number<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    many1::<String, _>(digit().or(token('.'))).map(|c| {
        c.parse::<f64>()
            .map(|f| Token::Number(f))
            .expect("Cannot parse float")
    })
}

pub fn def<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    string("def").map(|_| Token::Def)
}

pub fn extern_<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    string("extern").map(|_| Token::Extern)
}

pub fn ident<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    between(
        spaces(),
        spaces(),
        letter().or(token('_')).then(|d| {
            many::<String, _>(alpha_num().or(token('_'))).map(move |s| {
                let s = format!("{}{}", d, s);
                Token::Identifier(s)
            })
        }),
    )
}

pub fn eof<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    ::combine::eof().map(|_| Token::EOF)
}
