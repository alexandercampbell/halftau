use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ParenL,
    ParenR,
    BracketL,
    BracketR,
    Ident,
    IntLiteral,
    DoubleLiteral,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub text: String,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Elt {
    Int(i64),
    Double(f64),
    String_(String),
    Reference(String),
    List(Vec<Elt>),
    Vector(Vec<Elt>),
    Function {
        lexical_bindings: Vec<String>,
        body: Box<Elt>,
    },
    BuiltinFunction(String),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Double,
    String_,
    Reference,
    List,
    Vector,
    Function,
    Nil,
}

pub fn typeof_(v: &Elt) -> Type {
    match v {
        Elt::Int(_) => Type::Int,
        Elt::Double(_) => Type::Double,
        Elt::String_(_) => Type::String_,
        Elt::Reference(_) => Type::Reference,
        Elt::List(_) => Type::List,
        Elt::Vector(_) => Type::Vector,
        Elt::Function { .. } => Type::Function,
        Elt::BuiltinFunction { .. } => Type::Function,
        Elt::Nil => Type::Nil,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub bindings: HashMap<String, Elt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Runtime {
    pub root_scope: Scope,
}
