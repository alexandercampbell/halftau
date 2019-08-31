const TEST_PROGRAM: &'static str = r"
";

#[derive(Debug, PartialEq)]
enum Token {
    ParenL,
    ParenR,
    Ident(String),
    Grouper,
    Newline,
}

fn tokenize(s: String) -> Vec<Token> {
    use Token::*;

    let mut tokens: Vec<Token> = vec![];
    let mut current_ident = String::new();
    let mut line_no = 0;

    let ident_terminators = vec!['\n', ' ', '(', ')', '$'];

    for ch in s.chars() {
        if ident_terminators.contains(&ch) {
            // ident terminators

            if !current_ident.is_empty() {
                tokens.push(Ident(current_ident));
                current_ident = String::new();
            }

            match ch {
                '\n' => {
                    tokens.push(Newline);
                    line_no += 1;
                }
                '$' => {
                    tokens.push(Grouper);
                }
                '(' => {
                    tokens.push(ParenL);
                }
                ')' => {
                    tokens.push(ParenR);
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

fn main() {
    let tokens = tokenize(TEST_PROGRAM.to_string());
    println!("{:?}", tokens);
}

#[test]
fn test_inc_tokens() {
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
