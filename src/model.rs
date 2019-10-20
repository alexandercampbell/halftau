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
pub enum Node {
    FunctionCall(Vec<Node>),
    Vector(Vec<Node>),
    Reference(String),
    Int(i64),
    Double(f64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Double,
    String_,
    Function(Vec<Type>),
    Vector(Box<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i64),
    Double(f64),
    String_(String),
    Vector {
        type_: Type,
        elts: Vec<Value>,
    },
    Function {
        lexical_bindings: Vec<String>,
        type_sig: Vec<Type>,
        body: Vec<Value>,
    },
    BuiltinFunction {
        type_sig: Vec<Type>,
        exec: fn(Vec<Value>) -> Value,
    },
}

pub fn typeof_(v: &Value) -> Type {
    match v {
        Value::Int(_) => Type::Int,
        Value::Double(_) => Type::Double,
        Value::String_(_) => Type::String_,
        Value::Vector { type_, .. } => Type::Vector(Box::new(type_.clone())),
        Value::Function { type_sig, .. } => Type::Function(type_sig.clone()),
        Value::BuiltinFunction { type_sig, .. } => Type::Function(type_sig.clone()),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scope {
    pub parent: Option<Box<Scope>>,
    pub bindings: HashMap<String, Value>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Runtime {
    pub root_scope: Scope,
}
