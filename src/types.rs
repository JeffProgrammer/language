#[derive(Debug, PartialEq)]
pub enum Keyword {
    Int,
    Float,
    Let
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Equal,
    Colon,
    Keyword(Keyword),
    Identifier(String),
    Integer(i32),
    Float(f32)
}