extern crate combine;
extern crate kaleidscope;

use combine::Parser;
use kaleidscope::{ast, parser};

#[test]
fn parse_proto() {
    let mut p = parser::proto();
    let (proto, _) = p.parse("f(a, b, c)").unwrap();
    let ans = ast::Proto::new("f".into(), vec!["a".into(), "b".into(), "c".into()]);
    assert_eq!(proto, ans);
}

#[test]
fn parse_call() {
    let mut p = parser::call_expr();
    let (call, _) = p.parse("f(a, 1.0, b)").unwrap();
    println!("call = {:?}", call);
}

#[test]
fn parse_func() {
    let mut p = parser::func();
    let src = "def f(a, b) a";
    let (f, _) = p.parse(src).unwrap();
    println!("func = {:?}", f);
}

#[test]
fn parse_extern() {
    let mut p = parser::extern_();
    let src = "extern f(a, b) a";
    let (f, _) = p.parse(src).unwrap();
    println!("func = {:?}", f);
}
