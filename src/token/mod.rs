pub mod number;
pub use self::number::number;

#[derive(Debug)]
pub enum Token {
    EOF,
    Def,
    Extern,
    Identifier(String),
    Number(f64),
}
