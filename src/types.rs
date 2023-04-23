#[derive(Debug, PartialEq)]
pub enum Keyword {
    Int,
    Float,
    Let
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub line_number: i32,
    pub token: TokenType
}