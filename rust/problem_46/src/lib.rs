/*

Problem 46:

It was proposed by Christian Goldbach that every odd composite 
number can be written as the sum of a prime and twice a square.

9 = 7 + 2×12
15 = 7 + 2×22
21 = 3 + 2×32
25 = 7 + 2×32
27 = 19 + 2×22
33 = 31 + 2×12

It turns out that the conjecture was false.

What is the smallest odd composite that cannot be written as 
the sum of a prime and twice a square?

Soln description:

A set of odd composites is a subset of goldberg conjecture 
set (combinations of prime + 2 * sqr(x)) possibilities. 

To test conjecture, take set difference (values in set1 
not in set2). 

The results are 'failures' to disprove conjecture. 

*/


extern crate primes;

use std::collections::HashSet;
use primes::PrimeSet;


pub fn goldbachs_conjecture(limit: u64) -> HashSet<u64> {
    let mut set = HashSet::new();
    let squares = two_times_squares(limit);
    let primes = eligible_primes(limit);

    for i in squares.iter() {
        for j in primes.iter() {
            set.insert(i + j);
        }
    }
    set
}

pub fn eligible_primes(limit: u64) -> Vec<u64> {
    let mut pset1 = PrimeSet::new();
    pset1.iter().take_while(|&x| x < limit).collect::<Vec<_>>()
}

pub fn two_times_squares(limit: u64) -> Vec<u64> {
    (1..1000).map(|x| 2 * x * x).collect::<Vec<u64>>()
                                .into_iter()
                                .take_while(|&x| x < limit)
                                .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
