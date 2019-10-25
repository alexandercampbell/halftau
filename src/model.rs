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
    Macro,
    Car,
    Cdr,
    Cons,
    Empty_,
    If,
    Not,
    Nth,
    Plus,
    Minus,
    Mult,
    Div,
    Equal,
    Assert,
    AssertEq,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Elt {
    Int(i64),
    Double(f64),
    Bool(bool),
    String_(String),
    Symbol(String),
    List(Vec<Elt>),
    Vector(Vec<Elt>),
    Function {
        lexical_bindings: Vec<String>,
        body: Box<Elt>,
    },
    BuiltinFunction(Builtin),
    Macro {
        lexical_bindings: Vec<String>,
        body: Box<Elt>,
    },
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub bindings: HashMap<String, Elt>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Runtime {
    pub root_scope: Scope,
}
