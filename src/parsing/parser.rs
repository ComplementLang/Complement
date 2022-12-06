use num_bigint::BigInt;
use super::lexer::Token;
use super::Operator;

pub fn parse(tokens: &Vec<Token>) -> Option<AstNode> {
    parse_expression(tokens)
}

fn parse_expression(tokens: &[Token]) -> Option<AstNode> {
    let lowest = lowest_op(tokens);
    if let Some(lowest_index) = lowest {
        let op = match &tokens[lowest_index] {
            Token::Operator(op) => op,
            _ => panic!("Expected operator token")
        };
        let right = Box::new(parse_expression(&tokens[lowest_index + 1..])
            .expect("Failed to parse right side of expression"));
        return if op.arity() == 1 {
            Some(AstNode::UnaryExpression(*op, right))
        } else if let Some(left) = parse_expression(&tokens[..lowest_index]) {
            Some(AstNode::BinaryExpression(Box::new(left), *op, right))
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

fn parse_atom(tokens: &[Token]) -> Option<AstNode> {
    let token = tokens.first();
    if let Some(token) = token {
        let result = match token {
            Token::Number(n) => Some(AstNode::Number(n.clone())),
            Token::Identifier(v) => Some(AstNode::Variable(v.clone())),
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
        };
        result
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub enum AstNode {
    Number(BigInt),
    Variable(String),
    UnaryExpression(Operator, Box<AstNode>),
    BinaryExpression(Box<AstNode>, Operator, Box<AstNode>),
}

impl AstNode {
    pub fn walk(&self, f: &dyn Fn(&AstNode)) -> AstNode {
        f(self);
        match self {
            AstNode::Number(_) => self.clone(),
            AstNode::Variable(_) => self.clone(),
            AstNode::UnaryExpression(op, node) =>
                AstNode::UnaryExpression(*op, Box::new(node.walk(f))),
            AstNode::BinaryExpression(left, op, right) =>
                AstNode::BinaryExpression(
                    Box::new(left.walk(f)),
                    *op,
                    Box::new(right.walk(f)),
                )
        }
    }
}