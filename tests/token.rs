extern crate combine;
extern crate kaleidscope;

use combine::Parser;
use kaleidscope::token::*;

#[test]
fn parse_number() {
    let mut num = number();
    let (num, remain) = num.parse("1.234").unwrap();
    assert_eq!(num, Token::Number(1.234));
    assert_eq!(remain, "");
}

#[test]
fn parse_ident() {
    let mut p = ident();

    // single
    let (v, remain) = p.parse("v").unwrap();
    assert_eq!(v, Token::Identifier("v".into()));
    assert_eq!(remain, "");

    // with number
    let (v, remain) = p.parse("a12").unwrap();
    assert_eq!(v, Token::Identifier("a12".into()));
    assert_eq!(remain, "");

    // with under score
    let (v, remain) = p.parse("a_a").unwrap();
    assert_eq!(v, Token::Identifier("a_a".into()));
    assert_eq!(remain, "");

    // with space
    let (v, remain) = p.parse("a a").unwrap();
    assert_eq!(v, Token::Identifier("a".into()));
    assert_eq!(remain, "a");
}
