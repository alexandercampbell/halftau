use crate::model::*;
use std::collections::HashMap;

fn lookup(runtime: &Runtime, name: &String) -> Result<Elt, String> {
    for scope in runtime.scopes.iter() {
        if let Some(value) = scope.bindings.get(name) {
            return Ok(value.clone());
        }
    }
    Err(format!("variable {:?} undefined", name))
}

fn format_with_spaces(elts: &[Elt]) -> String {
    let mut s = String::new();
    for i in 0..elts.len() {
        s.push_str(&format_elt(&elts[i]));
        if i < elts.len() - 1 {
            s.push(' ')
        }
    }
    s
}

fn format_elt(elt: &Elt) -> String {
    match elt {
        Elt::List(items) => {
            let mut s = '('.to_string();
            s.push_str(&format_with_spaces(items));
            s.push(')');
            s
        }
        Elt::Vector(items) => {
            let mut s = '['.to_string();
            s.push_str(&format_with_spaces(items));
            s.push(']');
            s
        }
        Elt::Int(i) => format!("{}", i),
        Elt::Double(d) => format!("{}", d),
        Elt::String_(s) => s.clone(),
        Elt::Symbol(s) => s.clone(),
        Elt::Nil => format!("nil"),
        Elt::Function { .. } => format!("<function>"),
        Elt::BuiltinFunction(b) => format!("<builtin function {:?}>", b),
    }
}

fn eval_function(elts: &[Elt], runtime: &mut Runtime) -> Result<Elt, String> {
    if elts.len() == 0 {
        return Err("attempt to evaluate empty list as function".to_string());
    }

    let function = eval(&elts[0], runtime)?;

    match function {
        Elt::Function {
            ref lexical_bindings,
            ref body,
        } => {
            let mut args = vec![];
            for elt in &elts[1..] {
                args.push(eval(&elt, runtime)?);
            }

            if lexical_bindings.len() != args.len() {
                return Err(format!(
                    "{:?} expects {} parameters-- received {}",
                    function,
                    lexical_bindings.len(),
                    args.len()
                ));
            }

            let mut bindings = HashMap::new();
            let mut i = 0;
            while i < lexical_bindings.len() {
                bindings.insert(lexical_bindings[i].clone(), args[i].clone());
                i += 1;
            }

            let new_scope = Scope { bindings: bindings };
            let mut runtime = runtime.clone();
            runtime.scopes.push(new_scope);
            return eval(body, &mut runtime);
        }

        Elt::BuiltinFunction(btype) => {
            let args = &elts[1..];

            match btype {
                Builtin::Def => {
                    if args.len() != 2 {
                        return Err(format!("expected 2 arguments to def; {} found", args.len()));
                    }

                    if let Elt::Symbol(sym) = &args[0] {
                        let val = &eval(&args[1], runtime)?;
                        runtime.scopes[0].bindings.insert(sym.clone(), val.clone());
                        Ok(val.clone())
                    } else {
                        Err(format!("first parameter to def must be a symbol"))
                    }
                }
                Builtin::Print => {
                    for arg in args {
                        let elt = eval(&arg, runtime)?;
                        print!("{}", format_elt(&elt));
                        print!(" ");
                    }
                    Ok(Elt::Nil)
                }
                Builtin::Println => {
                    for arg in args {
                        let elt = eval(&arg, runtime)?;
                        print!("{}", format_elt(&elt));
                        print!(" ");
                    }
                    println!();
                    Ok(Elt::Nil)
                }
                Builtin::Quote => {
                    if args.len() != 1 {
                        return Err(format!(
                            "quote accepts only one parameter; {} found",
                            args.len()
                        ));
                    }
                    Ok(args[0].clone())
                }

                Builtin::Fn_ => {
                    if args.len() != 2 {
                        return Err(format!("fn requires 2 parameters; {} found", args.len()));
                    }

                    if let Elt::Vector(ref params) = args[0] {
                        let mut lexical_bindings = vec![];
                        for param in params {
                            if let Elt::Symbol(s) = param {
                                lexical_bindings.push(s.clone());
                            } else {
                                return Err("only symbols allowed in fn binding vector".to_string());
                            }
                        }

                        Ok(Elt::Function {
                            lexical_bindings,
                            body: Box::new(args[1].clone()),
                        })
                    } else {
                        Err("fn requires a vector of symbols as its first parameter".to_string())
                    }
                }
            }
        }

        _ => {
            return Err(format!("attempt to treat {:?} as function", function));
        }
    }
}

fn eval(value: &Elt, runtime: &mut Runtime) -> Result<Elt, String> {
    match value {
        Elt::List(elts) => eval_function(elts, runtime),
        Elt::Symbol(name) => eval(&lookup(runtime, &name)?, runtime),
        _ => Ok(value.clone()),
    }
}

pub fn execute(ast: Vec<Elt>) {
    let mut root_scope = Scope {
        bindings: HashMap::new(),
    };

    root_scope
        .bindings
        .insert("print".to_string(), Elt::BuiltinFunction(Builtin::Print));
    root_scope.bindings.insert(
        "println".to_string(),
        Elt::BuiltinFunction(Builtin::Println),
    );
    root_scope
        .bindings
        .insert("def".to_string(), Elt::BuiltinFunction(Builtin::Def));
    root_scope
        .bindings
        .insert("quote".to_string(), Elt::BuiltinFunction(Builtin::Quote));
    root_scope
        .bindings
        .insert("fn".to_string(), Elt::BuiltinFunction(Builtin::Fn_));

    let mut runtime = Runtime {
        scopes: vec![root_scope],
    };

    for node in ast {
        if let Err(e) = eval(&node, &mut runtime) {
            println!("error during evaluation: {}", e);
            break;
        }
    }
}
