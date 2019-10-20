use crate::model::*;
use std::collections::HashMap;

pub fn execute(ast: Vec<Node>) {
    let mut root_scope = Scope {
        parent: None,
        bindings: HashMap::new(),
    };

    root_scope.bindings.insert(
        "print_int".to_string(),
        Value::BuiltinFunction {
            type_sig: vec![Type::Int],
            exec: |args: Vec<Value>| -> Value {
                println!("{:?}", args[0]);
                args[0].clone()
            },
        },
    );

    let mut runtime = Runtime {
        root_scope: root_scope,
    };
}
