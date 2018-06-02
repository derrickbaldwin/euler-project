extern crate problem_29;
extern crate num;

use std::collections::HashSet;
use num::BigUint;
use num::pow::pow;

fn main() {
    let mut set = HashSet::new();
    for a in 2usize..101 {
        for b in 2usize..101 {
            set.insert(pow(BigUint::from(a), b));
        }
    }
    println!("{}", set.len());
}