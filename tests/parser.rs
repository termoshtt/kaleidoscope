extern crate combine;
extern crate kaleidscope;

use combine::Parser;
use kaleidscope::{ast, parser};

#[test]
fn parse_proto() {
    let mut p = parser::proto();
    let (proto, _) = p.parse("f(a, b)").unwrap();
    let ans = ast::Proto::new("f".into(), vec!["a".into(), "b".into()]);
    assert_eq!(proto, ans);
}
