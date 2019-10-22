use crate::model::*;
use std::collections::HashMap;

fn lookup(scope: &Scope, name: String) -> Result<Value, String> {
    if let Some(value) = scope.bindings.get(&name) {
        return Ok(value.clone());
    }
    if let Some(ref parent) = scope.parent {
        return lookup(parent, name);
    }
    Err(format!("variable {:?} undefined", name))
}

fn exec(value: Value, scope: &Scope) {
    if let Value::List(head, tail) = value {
        println!("execing {:?} on {:?}", head, tail);
    }

    /*
    match func {
        Value::Function {
            lexical_bindings,
            body,
        } => return Err(format!("call to user functions unimplemented")),

        Value::BuiltinFunction { arity, exec } => {
            if arity != args.len() {
                return Err(format!("attempt to call {:?} with the wrong arity", func));
            }
            return Ok(exec(args));
        }

        _ => Err(format!(
            "attempting to call {:?}, but it's not a function",
            func
        )),
    }
    */
}

fn eval(node: &Node, scope: &Scope) -> Result<Value, String> {
    match node {
        Node::Int(i) => Ok(Value::Int(i.clone())),
        Node::Double(d) => Ok(Value::Double(d.clone())),
        Node::Reference(name) => lookup(scope, name.clone()),
        Node::List(elts) => {
            if elts.len() == 0 {
                return Ok(Value::Unit);
            }

            let mut evaled = vec![];
            for e in elts.iter() {
                evaled.push(eval(e, scope)?);
            }
            evaled.reverse();

            let mut head = Value::List(Box::new(evaled[0].clone()), Box::new(Value::Unit));
            for e in evaled[1..].iter() {
                head = Value::List(Box::new(e.clone()), Box::new(head));
            }

            Ok(head)
        }
        _ => Err(format!("don't know how to process {:?}", node)),
    }
}

pub fn execute(ast: Vec<Node>) {
    let mut root_scope = Scope {
        parent: None,
        bindings: HashMap::new(),
    };

    root_scope.bindings.insert(
        "print".to_string(),
        Value::BuiltinFunction {
            exec: |args: Vec<Value>| -> Value {
                println!("{:?}", args[0]);
                args[0].clone()
            },
        },
    );

    for node in ast {
        let value = eval(&node, &root_scope).unwrap();
        exec(value, &root_scope);
        println!("{:?}", eval(&node, &root_scope).unwrap());
    }
}
