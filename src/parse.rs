use crate::model::Elt::*;
use crate::model::TokenType::*;
use crate::model::*;

fn parse_expr(tokens: &[Token], index: usize) -> Result<(Elt, usize), String> {
    match tokens.get(index) {
        Some(Token { _type: ParenL, .. }) => parse_list(tokens, index),
        Some(Token {
            _type: BracketL, ..
        }) => parse_vector(tokens, index),
        Some(Token {
            _type: IntLiteral,
            text,
            line_number,
        }) => match text.parse::<i64>() {
            Ok(i) => Ok((Int(i), index + 1)),
            Err(e) => Err(format!(
                "bad integer literal on line {}: {:?}: {}",
                line_number, text, e
            )),
        },
        Some(Token { _type: Quote, .. }) => {
            let (quoted, new_index) = parse_expr(tokens, index + 1)?;
            Ok((
                List(vec![BuiltinFunction("quote".to_string()), quoted]),
                new_index,
            ))
        }
        Some(Token {
            _type: StringLiteral,
            text,
            ..
        }) => Ok((String_(text.clone()), index + 1)),
        Some(Token {
            _type: DoubleLiteral,
            text,
            line_number,
        }) => match text.parse::<f64>() {
            Ok(i) => Ok((Double(i), index + 1)),
            Err(e) => Err(format!(
                "bad double literal on line {}: {:?}: {}",
                line_number, text, e
            )),
        },
        Some(Token {
            _type: Ident, text, ..
        }) => Ok((Symbol(text.clone()), index + 1)),
        Some(Token {
            _type: BracketR,
            line_number,
            ..
        }) => Err(format!(
            "unexpected closing bracket on line {}",
            line_number
        )),
        Some(Token {
            _type: ParenR,
            line_number,
            ..
        }) => Err(format!("unexpected closing paren on line {}", line_number)),
        None => Err("unexpected EOF".to_string()),
    }
}

fn parse_vector(tokens: &[Token], index: usize) -> Result<(Elt, usize), String> {
    let mut elts = vec![];
    assert_eq!(BracketL, tokens[index]._type);
    let start_line_number = tokens[index].line_number;

    // skip the opening bracket
    let mut index = index + 1;
    loop {
        if let Some(token) = tokens.get(index) {
            if token._type == BracketR {
                return Ok((Vector(elts), index + 1));
            }

            let (elt, new_index) = parse_expr(tokens, index)?;
            elts.push(elt);
            index = new_index;
        } else {
            return Err(format!(
                "unterminated vector starting on line {}",
                start_line_number
            ));
        }
    }
}

fn parse_list(tokens: &[Token], index: usize) -> Result<(Elt, usize), String> {
    let mut elts = vec![];
    assert_eq!(ParenL, tokens[index]._type);
    let start_line_number = tokens[index].line_number;

    // skip the lparen
    let mut index = index + 1;
    loop {
        if let Some(token) = tokens.get(index) {
            if token._type == ParenR {
                return Ok((List(elts), index + 1));
            }

            let (elt, new_index) = parse_expr(tokens, index)?;
            elts.push(elt);
            index = new_index;
        } else {
            return Err(format!(
                "unterminated function definition starting on line {}",
                start_line_number
            ));
        }
    }
}

pub fn parse(tokens: &[Token]) -> Result<Vec<Elt>, String> {
    let mut nodes: Vec<Elt> = vec![];
    let mut index = 0;
    while index < tokens.len() {
        let (node, new_index) = parse_expr(tokens, index)?;
        nodes.push(node);
        index = new_index;
    }
    Ok(nodes)
}
