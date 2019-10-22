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

fn eval(node: &Node, scope: &Scope) -> Result<Value, String> {
    match node {
        Node::Int(i) => Ok(Value::Int(i.clone())),
        Node::Double(d) => Ok(Value::Double(d.clone())),
        Node::Reference(name) => lookup(scope, name.clone()),
        Node::FunctionCall(elts) => {
            let func = eval(&elts[0], scope)?;
            let mut args = vec![];
            for e in elts[1..].iter() {
                args.push(eval(e, scope)?);
            }

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
            arity: 1,
            exec: |args: Vec<Value>| -> Value {
                println!("{:?}", args[0]);
                args[0].clone()
            },
        },
    );

    for node in ast {
        eval(&node, &root_scope).unwrap();
    }
}
