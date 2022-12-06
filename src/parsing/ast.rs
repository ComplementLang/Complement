use std::fmt::Debug;
use super::Operator;

#[derive(Debug)]
pub enum AstNode {
    Number(i64),
    Variable(String),
    UnaryExpression {
        operator: Operator,
        operand: Box<AstNode>
    },
    BinaryExpression {
        left: Box<AstNode>,
        operator: Operator,
        right: Box<AstNode>
    }
}