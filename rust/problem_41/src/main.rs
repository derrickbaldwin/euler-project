extern crate problem_41;
extern crate primes;
extern crate itertools;

use problem_41::pandigitals;
use itertools::chain;
use std::collections::BinaryHeap;
use primes::is_prime;

fn main() {
    let mut heap = BinaryHeap::new();
    for x in chain(pandigitals(4),pandigitals(7)) {
        heap.push(x);
    }
    let mut not_prime = true;
    let mut target = 0;
    while not_prime {
        let candidate = heap.pop().unwrap();
        if is_prime(candidate) {
            not_prime = false;
            target = candidate;
        }
    }
    println!("{}", target);
}
