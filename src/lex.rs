use crate::model::Token;
use crate::model::TokenType::*;

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

            '[' => tokens.push(Token {
                _type: BracketL,
                text: ch.to_string(),
                line_number,
            }),
            ']' => tokens.push(Token {
                _type: BracketR,
                text: ch.to_string(),
                line_number,
            }),
            '\'' => tokens.push(Token {
                _type: Quote,
                text: ch.to_string(),
                line_number,
            }),

            'a'..='z' | 'A'..='Z' | '+' | '-' | '*' | '/' | '=' | '?' => {
                let mut text = ch.to_string();
                while let Some(next) = chars.peek() {
                    if next.is_alphanumeric() || *next == '-' || *next == '\'' || *next == '?' {
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

            '\"' => {
                let mut text = String::new();
                while let Some(next) = chars.peek() {
                    if *next == '\"' {
                        chars.next();
                        break;
                    } else if *next == '\\' {
                        chars.next();
                        if let Some(escaped) = chars.peek() {
                            text.push(match escaped {
                                '\"' => '\"',
                                'n' => '\n',
                                't' => '\t',
                                _ => return Err(format!("unknown escape sequence \\{}", escaped)),
                            });
                            chars.next();
                        }
                        continue;
                    }
                    text.push(next.clone());
                    chars.next();
                }
                tokens.push(Token {
                    _type: StringLiteral,
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
