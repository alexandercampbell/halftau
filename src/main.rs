mod lex;
mod model;
mod parse;
mod runtime;

fn main() {
    use std::env;
    use std::fs::File;
    use std::io::prelude::*;

    let mut args = env::args();
    args.next();
    for arg in args {
        let mut file = File::open(arg).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let tokens = lex::lex(contents);
        match tokens {
            Ok(tokens) => {
                let ast = parse::parse(&tokens[..]);
                runtime::execute(ast.unwrap());
            }
            Err(s) => println!("Error: {}", s),
        }
    }
}
