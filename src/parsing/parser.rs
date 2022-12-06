use super::lexer::Token;
use super::ast::AstNode;

pub fn parse(tokens: &Vec<Token>) -> Option<AstNode> {
    parse_expression(tokens, &mut 0)
}

fn parse_expression(tokens: &[Token], index: &mut usize) -> Option<AstNode> {
    let next = try_parse_literal(tokens, index);
    return if let Some(next) = next {
        let token = tokens.get(*index);
        if let Some(Token::Operator(operator)) = token {
            *index += 1;
            let right = parse_expression(tokens, index);
            if let Some(right) = right {
                return Some(AstNode::BinaryExpression {
                    left: Box::new(next),
                    operator: *operator,
                    right: Box::new(right)
                });
            }
        }
        Some(next)
    } else {
        None
    }
}

fn try_parse_literal(tokens: &[Token], index: &mut usize) -> Option<AstNode> {
    let token = &tokens[*index];
    let result = match token {
        Token::Number(n) => Some(AstNode::Number(*n)),
        Token::Identifier(v) => Some(AstNode::Variable(v.clone())),
        _ => None
    };
    if result.is_some() {
        *index += 1;
    }
    result
}