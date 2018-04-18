pub mod identifier;
pub mod number;

pub use self::identifier::identifier;
pub use self::number::number;

#[derive(Debug, PartialEq)]
pub enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
}
