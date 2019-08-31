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
    let mut line_no = 0;

    let ident_terminators = vec!['\n', ' ', '(', ')', '$', '\t'];

    for ch in s.chars() {
        if ident_terminators.contains(&ch) {
            // ident terminators

            if !current_ident.is_empty() {
                tokens.push(Ident(current_ident));
                current_ident = String::new();
            }

            if ch == '\n' {
                line_no += 1;
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

    println!("Lexed {:?} lines", line_no);
    tokens
}

#[derive(Debug, PartialEq)]
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
        println!("considering {:?}", token);

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

fn main() {}

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
    println!("AST {:?}", ast);
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
