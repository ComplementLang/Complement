use core::clone::Clone;
use core::cmp::PartialEq;

use num_bigint::BigInt;

use super::Operator;

#[derive(Debug)]
pub enum Expression {
    Number(BigInt),
    Variable(String),
    UnaryExpression(Operator, Box<Expression>),
    BinaryExpression(Box<Expression>, Operator, Box<Expression>),
}

impl Expression {
    pub fn left(&self) -> &Expression {
        match self {
            Expression::BinaryExpression(left, _, _) => left,
            _ => panic!("Expected binary expression")
        }
    }

    pub fn right(&self) -> &Expression {
        match self {
            Expression::BinaryExpression(_, _, right) => right,
            Expression::UnaryExpression(_, right) => right,
            _ => panic!("Expected expression")
        }
    }

    pub fn operator(&self) -> &Operator {
        match self {
            Expression::BinaryExpression(_, op, _) => op,
            Expression::UnaryExpression(op, _) => op,
            _ => panic!("Expected expression")
        }
    }

    pub fn number(&self) -> &BigInt {
        match self {
            Expression::Number(n) => n,
            _ => panic!("Expected number")
        }
    }
}

impl PartialEq<Self> for Expression {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Expression::Number(n), Expression::Number(m)) => n == m,
            (Expression::Variable(v), Expression::Variable(w)) => v == w,
            (
                Expression::BinaryExpression(l1, o1, r1),
                Expression::BinaryExpression(l2, o2, r2)
            ) =>
                l1 == l2 && o1 == o2 && r1 == r2,
            (
                Expression::UnaryExpression(o1, r1),
                Expression::UnaryExpression(o2, r2)
            ) =>
                o1 == o2 && r1 == r2,
            _ => false
        }
    }
}

impl Clone for Expression {
    fn clone(&self) -> Self {
        match self {
            Expression::Number(n) => Expression::Number(n.clone()),
            Expression::Variable(v) => Expression::Variable(v.clone()),
            Expression::UnaryExpression(op, right) =>
                Expression::UnaryExpression(*op, Box::new(*right.clone())),
            Expression::BinaryExpression(left, op, right) =>
                Expression::BinaryExpression(
                    Box::new(*left.clone()),
                    *op,
                    Box::new(*right.clone())
                )
        }
    }
}
