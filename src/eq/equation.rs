use num_bigint::BigInt;

use crate::parsing::{Expression, Operator};

/// Currently assumes that there are no unary operators and that the variable is on one side
pub fn solve_equation(expr: &Expression) -> BigInt {
    let mut left;
    let mut right;
    if let Expression::BinaryExpression(l, Operator::Equal, r) = reduce(expr) {
        if has_var(&l) {
            left = *l;
            right = r.number().clone()
        } else if has_var(&r) {
            left = *r;
            right = l.number().clone();
        } else {
            panic!("No variable found in equation");
        }
    } else {
        panic!("Expected equation");
    }
    // By now the variable should be on the left side
    loop {
        let (l, r) = solve_step(&left, &right);
        if l == left {
            break;
        }
        left = l;
        right = r;
    }
    right
}


/// Performs one step of simplification. Expects the variable to be on the left side
fn solve_step(left: &Expression, right: &BigInt) -> (Expression, BigInt) {
    let var;
    let constant;
    let is_var_left;
    if let Expression::BinaryExpression(l, _, r) = left {
        if has_var(l) {
            var = l;
            constant = r.number();
            is_var_left = true;
        } else if has_var(r) {
            var = r;
            constant = l.number();
            is_var_left = false;
        } else {
            panic!("No variable found in equation");
        }
    } else {
        return (left.clone(), right.clone());
    }
    match left.operator() {
        Operator::Plus => (*var.clone(), right - constant),
        Operator::Minus => (*var.clone(), if is_var_left { right + constant } else { constant - right }),
        Operator::Multiply => (*var.clone(), right / constant),
        Operator::Divide => (*var.clone(), if is_var_left { right * constant } else { constant / right }),
        _ => panic!("Expected binary operator")
    }
}

/// Reduces an expression by one step
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

