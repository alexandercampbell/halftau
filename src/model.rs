use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ParenL,
    ParenR,
    BracketL,
    BracketR,
    Ident,
    IntLiteral,
    StringLiteral,
    DoubleLiteral,
    Quote,
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub _type: TokenType,
    pub text: String,
    pub line_number: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Builtin {
    Print,
    Println,
    Def,
    Quote,
    Fn_,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Elt {
    Int(i64),
    Double(f64),
    String_(String),
    Symbol(String),
    List(Vec<Elt>),
    Vector(Vec<Elt>),
    Function {
        lexical_bindings: Vec<String>,
        body: Box<Elt>,
    },
    BuiltinFunction(Builtin),
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Double,
    String_,
    Symbol,
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
        Elt::Symbol(_) => Type::Symbol,
        Elt::List(_) => Type::List,
        Elt::Vector(_) => Type::Vector,
        Elt::Function { .. } => Type::Function,
        Elt::BuiltinFunction(_) => Type::Function,
        Elt::Nil => Type::Nil,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub bindings: HashMap<String, Elt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Runtime {
    pub root_scope: Scope,
}
