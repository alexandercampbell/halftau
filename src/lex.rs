use crate::model::*;

pub fn lex(s: String) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut line_number = 1usize;

    let mut chars = s.chars().peekable();

    loop {
        let ch = chars.next();
        if ch.is_none() {
            break;
        }
        let ch = ch.unwrap();

        match ch {
            '\t' | ' ' => (),
            '\n' => line_number += 1,
            ';' => {
                while let Some(next) = chars.next() {
                    if next == '\n' {
                        line_number += 1;
                        break;
                    }
                }
            }
            '(' => tokens.push(Token {
                _type: ParenL,
                text: ch.to_string(),
                line_number,
            }),
            ')' => tokens.push(Token {
                _type: ParenR,
                text: ch.to_string(),
                line_number,
            }),

            'a'..='z' | 'A'..='Z' => {
                let mut text = ch.to_string();
                while let Some(next) = chars.peek() {
                    if next.is_alphanumeric() || *next == '-' || *next == '\'' {
                        text.push(*next);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    _type: Ident,
                    text,
                    line_number,
                });
            }

            '0'..='9' => {
                let mut text = ch.to_string();
                let mut is_double = false;

                while let Some(next) = chars.peek() {
                    if *next == '.' {
                        if is_double {
                            return Err(format!(
                                "multiple decimal places in float literal on line {}",
                                line_number
                            ));
                        }
                        is_double = true;
                        text.push(next.clone());
                        chars.next();
                        continue;
                    }
                    if next.is_numeric() {
                        text.push(next.clone());
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    _type: if is_double { DoubleLiteral } else { IntLiteral },
                    text,
                    line_number,
                });
            }

            _ => {
                return Err(format!(
                    "unrecognized character {:?} on line {}",
                    ch, line_number
                ));
            }
        }
    }

    Ok(tokens)
}
