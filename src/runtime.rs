use crate::model::*;
use std::collections::HashMap;

fn lookup(scope: &Scope, name: &String) -> Result<Elt, String> {
    if let Some(value) = scope.bindings.get(name) {
        return Ok(value.clone());
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

pub fn format_elt(elt: &Elt) -> String {
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
        Elt::Bool(b) => format!("{}", b),
        Elt::Int(i) => format!("{}", i),
        Elt::Double(d) => format!("{}", d),
        Elt::String_(s) => s.clone(),
        Elt::Symbol(s) => s.clone(),
        Elt::Nil => format!("nil"),
        Elt::Function { .. } => format!("<function>"),
        Elt::BuiltinFunction(b) => format!("<builtin function {:?}>", b),
        Elt::Macro { .. } => format!("<macro>"),
    }
}

fn replace_symbol(elt: &Elt, symbol: &str, value: Elt) -> Elt {
    match elt {
        Elt::Symbol(s) => {
            if s == symbol {
                value
            } else {
                elt.clone()
            }
        }
        Elt::List(l) => {
            let mut new = vec![];
            for item in l {
                new.push(replace_symbol(&item, symbol, value.clone()));
            }
            return Elt::List(new);
        }
        Elt::Vector(v) => {
            let mut new = vec![];
            for item in v {
                new.push(replace_symbol(&item, symbol, value.clone()));
            }
            return Elt::Vector(new);
        }
        _ => elt.clone(),
    }
}

fn eval_function(elts: &[Elt], runtime: &mut Runtime, scope: &Scope) -> Result<Elt, String> {
    if elts.len() == 0 {
        return Err("attempt to evaluate empty list as function".to_string());
    }

    let function = eval(&elts[0], runtime, scope)?;

    match function {
        Elt::Function {
            ref lexical_bindings,
            ref body,
        } => {
            let mut args = vec![];
            for elt in &elts[1..] {
                args.push(eval(&elt, runtime, scope)?);
            }

            if lexical_bindings.len() != args.len() {
                return Err(format!(
                    "{:?} expects {} parameters-- received {}",
                    function,
                    lexical_bindings.len(),
                    args.len()
                ));
            }

            let mut new_scope = scope.clone();
            let mut i = 0;
            while i < lexical_bindings.len() {
                new_scope
                    .bindings
                    .insert(lexical_bindings[i].clone(), args[i].clone());
                i += 1;
            }

            return eval(body, runtime, &new_scope);
        }

        Elt::BuiltinFunction(btype) => {
            let args = &elts[1..];

            match btype {
                Builtin::Def => {
                    if args.len() != 2 {
                        return Err(format!("expected 2 arguments to def; {} found", args.len()));
                    }

                    if let Elt::Symbol(sym) = &args[0] {
                        let val = &eval(&args[1], runtime, scope)?;
                        runtime.root_scope.bindings.insert(sym.clone(), val.clone());
                        Ok(val.clone())
                    } else {
                        Err(format!("first parameter to def must be a symbol"))
                    }
                }
                Builtin::Macro => {
                    if args.len() != 2 {
                        return Err(format!(
                            "expected 2 arguments to defmacro; {} found",
                            args.len()
                        ));
                    }

                    if let Elt::Vector(ref params) = args[0] {
                        let mut lexical_bindings = vec![];
                        for param in params {
                            if let Elt::Symbol(s) = param {
                                lexical_bindings.push(s.clone());
                            } else {
                                return Err(
                                    "only symbols allowed in macro binding vector".to_string()
                                );
                            }
                        }

                        Ok(Elt::Macro {
                            lexical_bindings,
                            body: Box::new(args[1].clone()),
                        })
                    } else {
                        Err(
                            "defmacro requires a vector of symbols as its first parameter"
                                .to_string(),
                        )
                    }
                }
                Builtin::Print => {
                    for arg in args {
                        let elt = eval(&arg, runtime, scope)?;
                        print!("{}", format_elt(&elt));
                        print!(" ");
                    }
                    Ok(Elt::Nil)
                }
                Builtin::Println => {
                    for arg in args {
                        let elt = eval(&arg, runtime, scope)?;
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

                Builtin::If => {
                    if args.len() < 2 || args.len() > 3 {
                        return Err(format!("if requires 2-3 parameters, found {}", args.len()));
                    }

                    let condition = eval(&args[0], runtime, scope)?;
                    match condition {
                        Elt::Bool(false) | Elt::Nil => {
                            if args.len() == 3 {
                                eval(&args[2], runtime, scope)
                            } else {
                                Ok(Elt::Bool(false))
                            }
                        }
                        _ => eval(&args[1], runtime, scope),
                    }
                }

                Builtin::Car => {
                    if args.len() != 1 {
                        return Err(format!(
                            "car takes only one parameter; {} found",
                            args.len()
                        ));
                    }

                    let list = eval(&args[0], runtime, scope)?;
                    if let Elt::List(ref elts) = list {
                        if elts.len() == 0 {
                            Err("attempt to car empty list".to_string())
                        } else {
                            Ok(elts[0].clone())
                        }
                    } else {
                        Err("car only accepts lists".to_string())
                    }
                }

                Builtin::Cdr => {
                    if args.len() != 1 {
                        return Err(format!(
                            "cdr takes only one parameter; {} found",
                            args.len()
                        ));
                    }

                    let list = eval(&args[0], runtime, scope)?;
                    if let Elt::List(ref elts) = list {
                        Ok(Elt::List(elts[1..].to_vec()))
                    } else {
                        Err("car only accepts lists".to_string())
                    }
                }

                Builtin::Cons => {
                    if args.len() != 2 {
                        return Err(format!("cons take two parameters; {} found", args.len()));
                    }

                    let first = eval(&args[0], runtime, scope)?;
                    let list = eval(&args[1], runtime, scope)?;
                    if let Elt::List(ref elts) = list {
                        let mut new_list = vec![first];
                        let mut old_list = elts.clone();
                        new_list.append(&mut old_list);
                        Ok(Elt::List(new_list))
                    } else {
                        return Err(format!("second arg to cons must be a list; got {:?}", list));
                    }
                }

                Builtin::Empty_ => {
                    if args.len() != 1 {
                        return Err(format!("empty? take one parameter; {} found", args.len()));
                    }

                    let list = eval(&args[0], runtime, scope)?;
                    if let Elt::List(ref elts) = list {
                        Ok(Elt::Bool(elts.len() == 0))
                    } else {
                        return Err(format!("arg to empty? must be a list; got {:?}", list));
                    }
                }

                Builtin::Nth => {
                    if args.len() != 2 {
                        return Err(format!("nth takes two parameters; {} found", args.len()));
                    }

                    let list = eval(&args[0], runtime, scope)?;
                    if let Elt::List(ref elts) = list {
                        let index = eval(&args[1], runtime, scope)?;
                        if let Elt::Int(i) = index {
                            match elts.get(i as usize) {
                                Some(v) => Ok(v.clone()),
                                _ => Err("index out of bounds".to_string()),
                            }
                        } else {
                            Err(format!(
                                "nth requires integer second param; got {:?}",
                                index
                            ))
                        }
                    } else {
                        Err("car only accepts lists".to_string())
                    }
                }

                Builtin::Plus => {
                    let mut is_double = false;
                    let mut acc_int = 0i64;
                    let mut acc_double = 0f64;

                    for arg in args {
                        match eval(&arg, runtime, scope)? {
                            Elt::Double(d) => {
                                if !is_double {
                                    acc_double = acc_int as f64;
                                    is_double = true;
                                }
                                acc_double += d;
                            }
                            Elt::Int(i) => {
                                if is_double {
                                    acc_double += i as f64;
                                } else {
                                    acc_int += i;
                                }
                            }
                            x => return Err(format!("attempt to perform addition on {:?}", x)),
                        }
                    }
                    if is_double {
                        Ok(Elt::Double(acc_double))
                    } else {
                        Ok(Elt::Int(acc_int))
                    }
                }

                Builtin::Minus => {
                    let mut first = true;
                    let mut is_double = false;
                    let mut acc_int = 0i64;
                    let mut acc_double = 0f64;

                    for arg in args {
                        match eval(&arg, runtime, scope)? {
                            Elt::Double(d) => {
                                if first {
                                    is_double = true;
                                    acc_double = d;
                                    first = false;
                                } else {
                                    if !is_double {
                                        acc_double = acc_int as f64;
                                        is_double = true;
                                    }
                                    acc_double -= d;
                                }
                            }
                            Elt::Int(i) => {
                                if first {
                                    acc_int = i;
                                    first = false;
                                } else {
                                    if is_double {
                                        acc_double -= i as f64;
                                    } else {
                                        acc_int -= i;
                                    }
                                }
                            }
                            x => return Err(format!("attempt to perform subtraction on {:?}", x)),
                        }
                    }
                    if is_double {
                        Ok(Elt::Double(acc_double))
                    } else {
                        Ok(Elt::Int(acc_int))
                    }
                }

                Builtin::Mult => {
                    let mut is_double = false;
                    let mut acc_int = 1i64;
                    let mut acc_double = 1f64;

                    for arg in args {
                        match eval(&arg, runtime, scope)? {
                            Elt::Double(d) => {
                                if !is_double {
                                    acc_double = acc_int as f64;
                                    is_double = true;
                                }
                                acc_double *= d;
                            }
                            Elt::Int(i) => {
                                if is_double {
                                    acc_double *= i as f64;
                                } else {
                                    acc_int *= i;
                                }
                            }
                            x => {
                                return Err(format!("attempt to perform multiplication on {:?}", x))
                            }
                        }
                    }
                    if is_double {
                        Ok(Elt::Double(acc_double))
                    } else {
                        Ok(Elt::Int(acc_int))
                    }
                }

                Builtin::Div => {
                    let mut first = true;
                    let mut acc_double = 0f64;

                    for arg in args {
                        match eval(&arg, runtime, scope)? {
                            Elt::Double(d) => {
                                if first {
                                    acc_double = d;
                                    first = false;
                                } else {
                                    acc_double /= d;
                                }
                            }
                            Elt::Int(i) => {
                                if first {
                                    acc_double = i as f64;
                                    first = false;
                                } else {
                                    acc_double /= i as f64;
                                }
                            }
                            x => return Err(format!("attempt to perform division on {:?}", x)),
                        }
                    }
                    Ok(Elt::Double(acc_double))
                }
            }
        }

        Elt::Macro {
            ref lexical_bindings,
            ref body,
        } => {
            let args = &elts[1..];
            if lexical_bindings.len() != args.len() {
                return Err(format!(
                    "{:?} expects {} parameters-- received {}",
                    function,
                    lexical_bindings.len(),
                    args.len()
                ));
            }

            let mut i = 0;
            let mut replaced: Elt = *body.clone();
            while i < lexical_bindings.len() {
                replaced = replace_symbol(&replaced, &lexical_bindings[i], args[i].clone());
                i += 1
            }

            return eval(&replaced, runtime, scope);
        }

        _ => {
            return Err(format!("attempt to treat {:?} as function", function));
        }
    }
}

pub fn eval(value: &Elt, runtime: &mut Runtime, scope: &Scope) -> Result<Elt, String> {
    match value {
        Elt::List(elts) => eval_function(elts, runtime, scope),
        Elt::Symbol(name) => lookup(scope, &name),
        _ => Ok(value.clone()),
    }
}

fn bind_builtins(b: &mut HashMap<String, Elt>) {
    b.insert("def".to_string(), Elt::BuiltinFunction(Builtin::Def));
    b.insert("quote".to_string(), Elt::BuiltinFunction(Builtin::Quote));
    b.insert("fn".to_string(), Elt::BuiltinFunction(Builtin::Fn_));
    b.insert("macro".to_string(), Elt::BuiltinFunction(Builtin::Macro));
    b.insert("car".to_string(), Elt::BuiltinFunction(Builtin::Car));
    b.insert("cdr".to_string(), Elt::BuiltinFunction(Builtin::Cdr));
    b.insert("cons".to_string(), Elt::BuiltinFunction(Builtin::Cons));
    b.insert("empty?".to_string(), Elt::BuiltinFunction(Builtin::Empty_));
    b.insert("if".to_string(), Elt::BuiltinFunction(Builtin::If));
    b.insert("nth".to_string(), Elt::BuiltinFunction(Builtin::Nth));
    b.insert("+".to_string(), Elt::BuiltinFunction(Builtin::Plus));
    b.insert("-".to_string(), Elt::BuiltinFunction(Builtin::Minus));
    b.insert("*".to_string(), Elt::BuiltinFunction(Builtin::Mult));
    b.insert("/".to_string(), Elt::BuiltinFunction(Builtin::Div));
    b.insert("print".to_string(), Elt::BuiltinFunction(Builtin::Print));
    b.insert(
        "println".to_string(),
        Elt::BuiltinFunction(Builtin::Println),
    );
}

pub fn execute(runtime: &mut Runtime, ast: Vec<Elt>) {
    for node in ast {
        let scope = runtime.root_scope.clone();
        if let Err(e) = eval(&node, runtime, &scope) {
            println!("error during evaluation: {}", e);
            break;
        }
    }
}

pub fn new() -> Runtime {
    let mut root_scope = Scope {
        bindings: HashMap::new(),
    };
    bind_builtins(&mut root_scope.bindings);
    Runtime { root_scope }
}
