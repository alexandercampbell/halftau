use crate::model::*;
use std::collections::HashMap;

fn lookup(scope: &Scope, name: String) -> Result<Elt, String> {
    if let Some(value) = scope.bindings.get(&name) {
        return Ok(value.clone());
    }
    if let Some(ref parent) = scope.parent {
        return lookup(parent, name);
    }
    Err(format!("variable {:?} undefined", name))
}

fn exec(value: Elt, scope: &Scope) {
    if let Elt::List(elts) = value {
        println!("execing {:?} on {:?}", elts[0], &elts[1..]);
    }

    /*
    match func {
        Elt::Function {
            lexical_bindings,
            body,
        } => return Err(format!("call to user functions unimplemented")),

        Elt::BuiltinFunction { arity, exec } => {
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

fn eval(node: &Elt, scope: &Scope) -> Result<Elt, String> {
    match node {
        Elt::Int(i) => Ok(Elt::Int(i.clone())),
        Elt::Double(d) => Ok(Elt::Double(d.clone())),
        Elt::Reference(name) => lookup(scope, name.clone()),
        Elt::List(elts) => {
            let mut evaled = vec![];
            for e in elts.iter() {
                evaled.push(eval(e, scope)?);
            }
            Ok(Elt::List(evaled))
        }
        _ => Err(format!("don't know how to process {:?}", node)),
    }
}

pub fn execute(ast: Vec<Elt>) {
    let mut root_scope = Scope {
        parent: None,
        bindings: HashMap::new(),
    };

    root_scope.bindings.insert(
        "print".to_string(),
        Elt::BuiltinFunction("print".to_string()),
    );

    for node in ast {
        let value = eval(&node, &root_scope).unwrap();
        exec(value, &root_scope);
        println!("{:?}", eval(&node, &root_scope).unwrap());
    }
}
