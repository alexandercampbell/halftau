#[derive(Debug, PartialEq)]
enum TokenType {
    ParenL,
    ParenR,

    Ident(String),
}
use TokenType::*;

#[derive(Debug, PartialEq)]
struct Token {
    _type: TokenType,
    line_number: usize,
}

fn tokenize(s: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut current_ident = String::new();
    let mut line_number = 1;
    let mut in_ident = false;

    for ch in s.chars() {
        if ch.is_alphabetic() && !in_ident {
            current_ident = String::new();
            current_ident.push(ch);
            in_ident = true;
        } else if in_ident && (ch.is_alphanumeric() || ch == '-' || ch == '_' || ch == '?') {
            current_ident.push(ch);
        } else {
            if in_ident {
                in_ident = false;
                tokens.push(Token {
                    _type: Ident(current_ident.clone()),
                    line_number: line_number,
                });
            }

            match ch {
                '(' => tokens.push(Token {
                    _type: ParenL,
                    line_number: line_number,
                }),
                ')' => tokens.push(Token {
                    _type: ParenR,
                    line_number: line_number,
                }),
                '\n' => line_number += 1,
                ' ' => (),
                '\t' => (),
                _ => {
                    return Err(format!(
                        "unrecognized character {:?} on line {}",
                        ch, line_number
                    ))
                }
            }
        }
    }

    Ok(tokens)
}

#[derive(Debug, PartialEq, Clone)]
enum Node {
    FunctionCall(Vec<Node>),
    Reference(String),
}
use Node::*;

fn parse_function_call(tokens: &[Token], index: usize) -> Result<(Node, usize), String> {
    let mut elts = vec![];
    let mut index = index;
    let start_line_number;

    println!("parsing function call {}", index);

    match tokens.get(index) {
        Some(Token {
            _type: ParenL,
            line_number: l,
        }) => {
            index += 1;
            start_line_number = l.clone();
        }

        Some(Token { line_number, .. }) => {
            return Err(format!(
                "expected open paren on line {}, got {:?}",
                line_number,
                tokens.get(index)
            ))
        }

        None => return Err(String::from("unexpected end of input")),
    };

    loop {
        let token = tokens.get(index);
        match token {
            Some(Token { _type: ParenR, .. }) => {
                println!("constructed function call with elts {:?}", elts);
                return Ok((FunctionCall(elts), index + 1));
            }
            Some(Token {
                _type: Ident(n), ..
            }) => {
                elts.push(Reference(n.clone()));
                index += 1;
            }
            Some(Token { _type: ParenL, .. }) => {
                let (node, new_index) = parse_function_call(tokens, index)?;
                elts.push(node);
                index = new_index;
            }
            None => {
                return Err(format!(
                    "unterminated function definition starting on line {}",
                    start_line_number
                ));
            }
        }
    }
}

fn parse(tokens: &[Token]) -> Result<Vec<Node>, String> {
    let mut nodes: Vec<Node> = vec![];
    let mut index = 0;
    while index < tokens.len() {
        match parse_function_call(tokens, index) {
            Ok((node, new_index)) => {
                nodes.push(node);
                index = new_index;
            }
            Err(s) => {
                return Err(s);
            }
        }
    }
    Ok(nodes)
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
        match tokens {
            Ok(tokens) => {
                println!("Tokens: {:?}", tokens);
                let ast = parse(&tokens[..]);
                println!("AST: {:?}", ast);
            }
            Err(s) => println!("Error: {}", s),
        }
    }
}
