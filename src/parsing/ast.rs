use std::fmt::Debug;
use super::Operator;

#[derive(Debug)]
pub enum AstNode {
    Number(i64),
    Variable(String),
    UnaryExpression(Operator, Box<AstNode>),
    BinaryExpression(Box<AstNode>, Operator, Box<AstNode>)
}