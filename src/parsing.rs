pub mod lexer;
pub mod parser;
pub mod ast;

#[derive(Debug, Copy, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,

    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
    Or,
    And,
    Not,
    Implies,

    Contains,
    NotContains,
    Subset,
    StrictSubset,
    Superset,
    StrictSuperset,
    Union,
    Intersection,
    Difference,
    SymmetricDifference
}

impl Operator {
    pub fn from_char(c: &char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '%' => Some(Operator::Modulo),
            '^' => Some(Operator::Exponent),

            '=' => Some(Operator::Equal),
            '≠' => Some(Operator::NotEqual),
            '<' => Some(Operator::LessThan),
            '>' => Some(Operator::GreaterThan),
            '≤' => Some(Operator::LessThanEqual),
            '≥' => Some(Operator::GreaterThanEqual),
            '∨' => Some(Operator::Or),
            '∧' => Some(Operator::And),
            '¬' => Some(Operator::Not),
            '⇒' => Some(Operator::Implies),

            '∈' => Some(Operator::Contains),
            '∉' => Some(Operator::NotContains),
            '⊆' => Some(Operator::Subset),
            '⊂' => Some(Operator::StrictSubset),
            '⊇' => Some(Operator::Superset),
            '⊃' => Some(Operator::StrictSuperset),
            '∪' => Some(Operator::Union),
            '∩' => Some(Operator::Intersection),
            '\\' => Some(Operator::Difference),
            '∆' => Some(Operator::SymmetricDifference),
            _ => None
        }
    }

    fn arity(&self) -> i32 {
        match self {
            Operator::Not => 1,
            _ => 2
        }
    }
}