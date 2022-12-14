use crate::parsing::expr::Expression;

use super::lexer::Token;

pub fn parse(tokens: &Vec<Token>) -> Option<Expression> {
    parse_expression(tokens)
}

fn parse_expression(tokens: &[Token]) -> Option<Expression> {
    let lowest = lowest_op(tokens);
    if let Some(lowest_index) = lowest {
        let op = match &tokens[lowest_index] {
            Token::Operator(op) => op,
            _ => panic!("Expected operator token")
        };
        let right = Box::new(parse_expression(&tokens[lowest_index + 1..])
            .expect("Failed to parse right side of expression"));
        return if op.arity() == 1 {
            Some(Expression::UnaryExpression(*op, right))
        } else if let Some(left) = parse_expression(&tokens[..lowest_index]) {
            Some(Expression::BinaryExpression(Box::new(left), *op, right))
        } else {
            None
        };
    } else {
        parse_atom(tokens)
    }
}

/// Returns the index of the rightmost operator with the lowest precedence
fn lowest_op(tokens: &[Token]) -> Option<usize> {
    let mut lowest = i32::MAX;
    let mut lowest_index = 0;
    let mut depth = 0;
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Operator(op) => {
                // <= because we want the rightmost operator
                if depth == 0 && op.precedence() <= lowest {
                    lowest = op.precedence();
                    lowest_index = i;
                }
            }
            Token::OpenParen => depth += 1,
            Token::CloseParen => depth -= 1,
            _ => {}
        }
    }
    if lowest == i32::MAX { None } else { Some(lowest_index) }
}

fn parse_atom(tokens: &[Token]) -> Option<Expression> {
    let token = tokens.first();
    if let Some(token) = token {
        match token {
            Token::Number(n) => Some(Expression::Number(n.clone())),
            Token::Identifier(v) => Some(Expression::Variable(v.clone())),
            Token::OpenParen => {
                let mut depth = 1;
                let mut i = 1;
                while depth > 0 {
                    match tokens[i] {
                        Token::OpenParen => depth += 1,
                        Token::CloseParen => depth -= 1,
                        _ => {}
                    }
                    i += 1;
                }
                parse_expression(&tokens[1..i - 1])
            }
            _ => None
        }
    } else {
        None
    }
}