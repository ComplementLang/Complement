use crate::parsing::{Expression, Operator};

/// Currently assumes that there are no unary operators
pub fn solve_equation(expr: &Expression) -> Expression {
    let mut result = expr.clone();
    loop {
        let new_result = reduce(&result);
        if new_result == result {
            break;
        }
        result = new_result;
    }
    result
}

fn reduce(expr: &Expression) -> Expression {
    match expr {
        Expression::BinaryExpression(l, o, r) => {
            let left = reduce(l);
            let right = reduce(r);
            match o {
                Operator::Plus => {
                    if let Expression::Number(ref n) = left {
                        if let Expression::Number(m) = right {
                            return Expression::Number(n + m);
                        }
                    }
                }
                Operator::Minus => {
                    if let Expression::Number(ref n) = left {
                        if let Expression::Number(m) = right {
                            return Expression::Number(n - m);
                        }
                    }
                }
                Operator::Multiply => {
                    if let Expression::Number(ref n) = left {
                        if let Expression::Number(m) = right {
                            return Expression::Number(n * m);
                        }
                    }
                }
                Operator::Divide => {
                    if let Expression::Number(ref n) = left {
                        if let Expression::Number(m) = right {
                            return Expression::Number(n / m);
                        }
                    }
                }
                _ => {}
            }
            Expression::BinaryExpression(
                Box::new(left),
                *o,
                Box::new(right),
            )
        }
        _ => expr.clone()
    }
}

fn has_var(expr: &Expression) -> bool {
    match expr {
        Expression::Variable(_) => true,
        Expression::BinaryExpression(left, _, right) =>
            has_var(left) || has_var(right),
        _ => false
    }
}

