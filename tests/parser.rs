extern crate combine;
extern crate kaleidscope;

use combine::Parser;
use kaleidscope::{ast, parser};

#[test]
fn parse_binary() {
    let mut p = parser::binary_expr();
    let src = "a + b + ( c + d ) * e * f + g";
    let (bin, remain) = p.parse(src).unwrap();
    println!("bin = {:?}", bin);
    assert_eq!(remain, "");
}

#[test]
fn parse_proto() {
    let mut p = parser::proto();
    let (proto, remain) = p.parse("f(a, b, c)").unwrap();
    let ans = ast::Proto::new("f".into(), vec!["a".into(), "b".into(), "c".into()]);
    assert_eq!(proto, ans);
    assert_eq!(remain, "");
}

#[test]
fn parse_call() {
    let mut p = parser::call_expr();
    let (call, remain) = p.parse("f(a, 1.0, b)").unwrap();
    println!("call = {:?}", call);
    assert_eq!(remain, "");
}

#[test]
fn parse_call2() {
    let mut p = parser::call_expr();
    let (call, remain) = p.parse("f(a, f(b))").unwrap();
    println!("call = {:?}", call);
    println!("remain = {:?}", remain);
    assert_eq!(remain, "");
}

#[test]
fn parse_func() {
    let mut p = parser::func();
    let src = "def f(a, b) a";
    let (f, remain) = p.parse(src).unwrap();
    println!("func = {:?}", f);
    assert_eq!(remain, "");
}

#[test]
fn parse_extern() {
    let mut p = parser::extern_();
    let src = "extern f(a, b) a";
    let (f, remain) = p.parse(src).unwrap();
    println!("func = {:?}", f);
    assert_eq!(remain, "");
}
