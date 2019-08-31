use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Token {
    ParenL,
    ParenR,
    Ident(String),
    Grouper,
    Newline,
    Tab,
}
use Token::*;

fn tokenize(s: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_ident = String::new();

    let ident_terminators = vec!['\n', ' ', '(', ')', '$', '\t'];

    for ch in s.chars() {
        if ident_terminators.contains(&ch) {
            // ident terminators

            if !current_ident.is_empty() {
                tokens.push(Ident(current_ident));
                current_ident = String::new();
            }

            let maybe_token = match ch {
                '\n' => Some(Newline),
                '\t' => Some(Tab),
                '$' => Some(Grouper),
                '(' => Some(ParenL),
                ')' => Some(ParenR),
                _ => None,
            };
            match maybe_token {
                Some(t) => {
                    tokens.push(t);
                }
                _ => (),
            }
        } else {
            current_ident.push(ch);
        }
    }

    tokens
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
    FunctionCall(Vec<Node>),
    Variable(String),
}
use Node::*;

fn parse_function_call(tokens: &[Token]) -> (Node, &[Token]) {
    let mut params: Vec<Node> = vec![];
    let mut tokens = tokens;

    loop {
        match tokens.get(0) {
            Some(Ident(name)) => {
                params.push(Variable(name.to_string()));
                tokens = &tokens[1..];
            }
            Some(Grouper) => {
                let (final_param, tail) = parse_function_call(&tokens[1..]);
                params.push(final_param);
                tokens = tail;
            }
            Some(Newline) => match tokens.get(1) {
                Some(Tab) => {
                    tokens = &tokens[2..];
                }
                _ => {
                    tokens = &tokens[1..];
                    break;
                }
            },
            Some(k) => {
                panic!("unrecognized token {:?}", k);
            }
            None => {
                break;
            }
        }
    }
    (FunctionCall(params), tokens)
}

fn parse(tokens: &[Token]) -> Vec<Node> {
    let mut nodes: Vec<Node> = vec![];
    let mut tokens = tokens;

    loop {
        let token = tokens.get(0);
        if token.is_none() {
            break;
        }
        let token = token.unwrap();

        match token {
            Ident(_) => {
                let (node, remaining) = parse_function_call(&tokens);
                nodes.push(node);
                tokens = remaining;
            }

            Tab => tokens = &tokens[1..],
            Newline => tokens = &tokens[1..],
            _ => panic!("unrecognized token {:?}", token),
        }
    }
    nodes
}

#[derive(Debug)]
enum Function {
    Defintion,
    UserDefined { arguments: Vec<Node>, body: Node },
}

#[derive(Debug)]
struct Runtime {
    definitions: HashMap<String, Function>,
}

impl Runtime {
    fn resolve_function(&self, node: &Node) -> Function {
        match node {
            Variable(text) => match &text[..] {
                "fn" => Function::Defintion {},
                _ => panic!("unrecognized function name"),
            },
            _ => panic!("function call attemped with non-name"),
        }
    }

    fn execute_function(&mut self, func: &Function, arguments: &[Node]) {
        println!(
            "executing function call {:?} with args {:?}",
            func, arguments
        );

        match func {
            Function::Defintion => {
                let name = match arguments.get(0) {
                    Some(Variable(text)) => text,
                    Some(_) => panic!("fn can only accept a variable name as its first parameter"),
                    None => panic!("insufficient parameters to fn"),
                };
                println!("defining new function {:?}", name);

                assert!(arguments.len() >= 2);
                let function_arguments = &arguments[1..arguments.len() - 1];
                let body = arguments[1..].get(arguments.len() - 2).unwrap();

                self.definitions.insert(
                    name.to_string(),
                    Function::UserDefined {
                        arguments: function_arguments.to_vec(),
                        body: body.clone(),
                    },
                );
            }

            Function::UserDefined { .. } => {}
        }
    }
}

fn execute(program: Vec<Node>) {
    let mut runtime = Runtime {
        definitions: HashMap::new(),
    };

    for node in program {
        match node {
            FunctionCall(nodes) => {
                assert!(nodes.len() > 0);
                let func = runtime.resolve_function(&nodes[0]);
                runtime.execute_function(&func, &nodes[1..]);
            }
            _ => panic!("only function calls are allowed at the top level"),
        }
    }

    println!("created definitions: {:?}", runtime);
}

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

        let tokens = tokenize(contents);
        let ast = parse(&tokens[..]);
        execute(ast);
    }
}

#[test]
fn test_inc_program() {
    use Token::*;

    let tokens = tokenize(
        "

        fn inc x $
        \t+ x 1

        "
        .to_string(),
    );
    assert!(
        tokens
            == vec![
                Newline,
                Newline,
                Ident("fn".to_string()),
                Ident("inc".to_string()),
                Ident("x".to_string()),
                Grouper,
                Newline,
                Tab,
                Ident("+".to_string()),
                Ident("x".to_string()),
                Ident("1".to_string()),
                Newline,
                Newline,
            ]
    );

    let ast = parse(tokens.as_slice());
    assert!(
        ast == vec![FunctionCall(vec![
            Variable("fn".to_string()),
            Variable("inc".to_string()),
            Variable("x".to_string()),
            FunctionCall(vec![
                Variable("+".to_string()),
                Variable("x".to_string()),
                Variable("1".to_string())
            ]),
        ])]
    );
}
