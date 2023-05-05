#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Int,
    Float,
    Let,
}

#[derive(Debug, PartialEq, Clone)]
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
    Float(f32),
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub line_number: i32,
    pub token: TokenType,
}

#[derive(Debug, PartialEq)]
pub enum IntOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
}

#[derive(Debug, PartialEq)]
pub enum FloatOperation {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
pub enum VariableType {
    Int,
    Float,
}

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    VariableAssignment {
        name: String,
        var_type: Option<VariableType>,
        expression: Option<Box<ASTNode>>,
    },
    LiteralInteger {
        value: i32,
    },
    LiteralFloat {
        value: f32,
    },
    IntOp {
        op: IntOperation,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    FloatOp {
        op: FloatOperation,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
}
