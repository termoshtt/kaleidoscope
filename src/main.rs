extern crate combine;
mod token;
use combine::Parser;

fn main() {
    let mut num = token::number();
    println!("{:?}", num.parse("1.234"));
    println!("{:?}", num.parse("1234"));
}
