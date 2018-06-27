/*
Problem 52: 

It can be seen that the number, 125874, and its 
double, 251748, contain exactly the same digits, 
but in a different order.

Find the smallest positive integer, x, such that 
2x, 3x, 4x, 5x, and 6x, contain the same digits.

*/

extern crate itertools;

use itertools::sorted;
use std::collections::HashSet;


#[derive(Debug)]
pub struct Multiple {
    pub seed: u64,
    pub multiples: Vec<String>,
}

impl Multiple {
    pub fn is_permuted(&mut self) -> bool {
        (self.multiples.iter().map(|x| sorted(x.chars().collect::<Vec<_>>())
                            .iter().collect::<String>())
                            .collect::<HashSet<String>>()).len() == 1
    }
}

pub fn build_multiples(seed: u64, n:u64) -> Multiple {
    let nums = (1..n+1).map(|x| (x * seed).to_string())
                           .collect::<Vec<_>>();
    Multiple {
        seed: seed,
        multiples: nums.clone(),
    }
}

pub fn find_first_multiple(n: u64) -> Option<u64> {
    (1..).find(|&x| build_multiples(x, n).is_permuted())
}


#[cfg(test)]
mod tests {
    use find_first_multiple;

    #[test]
    fn multiples_2() {
        assert_eq!(find_minimum_multiples(2), Some(125874));
    }
}
