use std::fmt::Debug;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Negate,

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
    SymmetricDifference,
    Size
}

impl Operator {
    pub fn from_char(c: &char) -> Option<Operator> {
        match c {
            '+' => Some(Operator::Plus),
            '-' => Some(Operator::Minus),
            '*' => Some(Operator::Multiply),
            '/' => Some(Operator::Divide),
            '%' => Some(Operator::Modulo),
            '¯' => Some(Operator::Negate),

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
            '#' => Some(Operator::Size),
            _ => None
        }
    }

    pub fn arity(&self) -> i32 {
        match self {
            Operator::Not | Operator::Negate | Operator::Size => 1,
            _ => 2
        }
    }

    pub fn precedence(&self) -> i32 {
        match self {
            Operator::Contains | Operator::NotContains | Operator::Subset | Operator::StrictSubset
            | Operator::Superset | Operator::StrictSuperset | Operator::Union | Operator::Intersection
            | Operator::Difference | Operator::SymmetricDifference => 1,

            Operator::Or | Operator::And | Operator::Implies => 2,

            Operator::Equal | Operator::NotEqual | Operator::LessThan | Operator::GreaterThan
            | Operator::LessThanEqual | Operator::GreaterThanEqual => 3,

            Operator::Plus | Operator::Minus => 4,

            Operator::Multiply | Operator::Divide | Operator::Modulo => 5,

            Operator::Not | Operator::Negate | Operator::Size => 7
        }
    }
}