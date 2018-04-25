use combine::char::{alpha_num, digit, letter, spaces, string};
use combine::*;

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
}

fn number<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    many1::<String, _>(digit().or(token('.'))).map(|c| {
        c.parse::<f64>()
            .map(|f| Token::Number(f))
            .expect("Cannot parse float")
    })
}

fn def<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    string("def").map(|_| Token::Def)
}

fn extern_<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    string("extern").map(|_| Token::Extern)
}

fn ident<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    letter().then(|d| {
        many::<String, _>(alpha_num()).map(move |s| {
            let s = format!("{}{}", d, s);
            Token::Identifier(s)
        })
    })
}

fn words<I: Stream<Item = char>>() -> impl Parser<Input = I, Output = Token> {
    def().or(extern_().or(ident()))
}

pub fn parse(mut cur: &str) -> Vec<Token> {
    let mut p = spaces().then(|_| eof().map(|_| Token::EOF).or(number().or(words())));
    let mut tokens = Vec::new();
    loop {
        let (t, remain) = p.parse(cur).expect("Failed to parse");
        if t == Token::EOF {
            break;
        }
        tokens.push(t);
        cur = remain;
    }
    tokens.push(Token::EOF);
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_words() {
        let mut p = words();
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
    fn parse_tokens() {
        let input = "def a 1.23";
        let tokens = parse(input);
        let ans = vec![
            Token::Def,
            Token::Identifier("a".into()),
            Token::Number(1.23),
            Token::EOF,
        ];
        assert_eq!(tokens, ans);
    }
}
