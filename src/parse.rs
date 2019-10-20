use crate::model::*;

fn parse_expr(tokens: &[Token], index: usize) -> Result<(Node, usize), String> {
    match tokens.get(index) {
        Some(Token { _type: ParenL, .. }) => parse_function_call(tokens, index),
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
        }) => Ok((Reference(text.clone()), index + 1)),
        Some(Token {
            _type: ParenR,
            line_number,
            ..
        }) => Err(format!("unexpected closing paren on line {}", line_number)),
        None => Err("unexpected EOF".to_string()),
    }
}

fn parse_function_call(tokens: &[Token], index: usize) -> Result<(Node, usize), String> {
    let mut elts = vec![];
    let mut index = index;
    let start_line_number;

    println!("parsing function call at index {}", index);

    match tokens.get(index) {
        Some(Token {
            _type: ParenL,
            line_number: l,
            ..
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
        if let Some(token) = tokens.get(index) {
            if token._type == ParenR {
                return Ok((FunctionCall(elts), index + 1));
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

pub fn parse(tokens: &[Token]) -> Result<Vec<Node>, String> {
    let mut nodes: Vec<Node> = vec![];
    let mut index = 0;
    while index < tokens.len() {
        let (node, new_index) = parse_function_call(tokens, index)?;
        nodes.push(node);
        index = new_index;
    }
    Ok(nodes)
}
