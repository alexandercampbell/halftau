#[derive(Debug, PartialEq)]
pub enum TokenType {
    ParenL,
    ParenR,
    Ident,
    IntLiteral,
    DoubleLiteral,
}
pub use TokenType::*;

#[derive(Debug, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub text: String,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    FunctionCall(Vec<Node>),
    Reference(String),
    Int(i64),
    Double(f64),
}
pub use Node::*;
