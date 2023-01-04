use num_bigint::BigInt;

use crate::constrainer::Constraint;

pub type BruteForceDegree = Box<dyn Iterator<Item = BigInt>>;

pub fn bundle(constraints: Vec<Constraint>) -> Vec<BruteForceDegree> { unimplemented!(); }
