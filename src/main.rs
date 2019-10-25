mod lex;
mod model;
mod parse;
mod runtime;

const PRELUDE: &str = include_str!("../prelude.tau");

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    let mut args = env::args();
    let mut runtime = runtime::new();

    runtime::execute(
        &mut runtime,
        parse::parse(&lex::lex(PRELUDE.to_string()).unwrap()).unwrap(),
    );

    args.next();
    for arg in args {
        let mut file = File::open(arg).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let tokens = lex::lex(contents);
        match tokens {
            Ok(tokens) => {
                let ast = parse::parse(&tokens[..]);
                runtime::execute(&mut runtime, ast.unwrap());
            }
            Err(s) => println!("Error: {}", s),
        }
    }
}
