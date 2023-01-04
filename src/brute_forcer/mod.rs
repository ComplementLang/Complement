use num_bigint::BigInt;

use crate::bundler::BruteForceDegree;

pub enum SolutionOrNot {
    Solution(Vec<BigInt>),
    Contradiction(String)
}

pub fn brute_force(bundle: Vec<BruteForceDegree>) -> SolutionOrNot { unimplemented!(); }
