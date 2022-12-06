use super::lexer::Token;
use super::ast::AstNode;

pub fn parse(tokens: &[Token]) -> AstNode {
    parse_expression(tokens, &mut 0)
}

fn parse_expression(tokens: &[Token], index: &mut usize) -> AstNode {
    let next = &tokens[*index];
    return match next {
        Token::Number(number) => AstNode::Number(*number),
        Token::Identifier(variable) => AstNode::Variable(variable.clone()),
        Token::Operator(operator) => {
            // we have a unary operator
            if operator.arity() == 1 {
                AstNode::UnaryExpression(*operator, Box::new(parse_expression(tokens, index)))
            } else {
                panic!("Invalid binary operator");
            }
        }
        Token::OpenBrace => todo!(),
        Token::CloseBrace => panic!("Invalid closing brace"),
        Token::OpenParen => {
            *index += 1;
            let expression = parse_expression(tokens, index);
            *index += 1;
            expression
        },
        Token::CloseParen => panic!("Invalid closing parenthesis"),
        Token::Quote => todo!()
    }
}