const TEST_PROGRAM: &'static str = r"
";

#[derive(Debug, PartialEq)]
enum Token {
    ParenL,
    ParenR,
    Ident(String),
    Grouper,
    Newline,
    Tab,
}

fn tokenize(s: String) -> Vec<Token> {
    use Token::*;

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
    println!("Parsed {:?} lines", line_no);
    tokens
}

// fn parse(tokens: Vec<Token>) {}

fn main() {
    let tokens = tokenize(TEST_PROGRAM.to_string());
    println!("{:?}", tokens);
}

#[test]
fn test_inc_program() {
    use Token::*;

    let tokens = tokenize(
        "

        inc x $ + x 1

        "
        .to_string(),
    );
    println!("{:?}", tokens);
    assert!(
        tokens
            == vec![
                Newline,
                Newline,
                Ident("inc".to_string()),
                Ident("x".to_string()),
                Grouper,
                Ident("+".to_string()),
                Ident("x".to_string()),
                Ident("1".to_string()),
                Newline,
                Newline,
            ]
    );
}
