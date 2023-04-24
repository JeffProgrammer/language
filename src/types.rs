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
    Float
}

#[derive(Debug, PartialEq)]
pub enum NodeType {
    StatementOp,
    VariableAssignment(String, Option<VariableType>),
    LiteralInteger(i32),
    LiteralFloat(f32),
    IntOp(IntOperation),
    FloatOp(FloatOperation)
}

#[derive(Debug, PartialEq)]
pub struct TreeNode {
    pub leaf: NodeType,
    pub left_branch: Option<Box<TreeNode>>,
    pub right_branch: Option<Box<TreeNode>>
}