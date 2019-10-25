mod lex;
mod model;
mod parse;
mod runtime;

const PRELUDE: &str = include_str!("../prelude.tau");

fn main() {
    use std::env;
    use std::fs::File;
    use std::io;
    use std::io::prelude::*;

    let mut args = env::args();
    let mut runtime = runtime::new();

    runtime::execute(
        &mut runtime,
        parse::parse(&lex::lex(PRELUDE.to_string()).unwrap()).unwrap(),
    );

    if args.len() == 1 {
        // repl
        let stdin = io::stdin();
        let mut lines = stdin.lock().lines();
        loop {
            print!("\u{03BB} ");
            io::stdout().flush().unwrap();
            let line = match lines.next() {
                Some(l) => l,
                _ => break,
            };
            let tokens = match lex::lex(line.unwrap()) {
                Ok(t) => t,
                Err(e) => {
                    println!("lexer error: {}", e);
                    continue;
                }
            };

            let ast = match parse::parse(&tokens) {
                Ok(a) => a,
                Err(e) => {
                    println!("parse error: {}", e);
                    continue;
                }
            };

            let scope = runtime.root_scope.clone();
            for node in ast {
                match runtime::eval(&node, &mut runtime, &scope) {
                    Ok(elt) => println!("{}", runtime::format_elt(&elt)),
                    Err(e) => println!("error: {:#?}", e),
                }
            }
        }
    } else {
        args.next();
        for arg in args {
            let mut file = File::open(&arg).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let tokens = lex::lex(contents);
            match tokens {
                Ok(tokens) => {
                    let ast = parse::parse(&tokens);
                    runtime::execute(&mut runtime, ast.unwrap());
                }
                Err(s) => println!("Lexer error in {}: {}", arg, s),
            }
        }
    }
}
