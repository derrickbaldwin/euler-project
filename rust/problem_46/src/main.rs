extern crate problem_46;
extern crate primes;

use problem_46::goldbachs_conjecture;
use primes::is_prime;
use std::collections::HashSet;


fn main() {
    let limit = 10_000;
    let odd_composites = (9u64..limit).filter(|&x| !is_prime(x) && x % 2 != 0)
                                      .collect::<HashSet<_>>();
    let goldbachs_conjecture_table = goldbachs_conjecture(limit);                                  

    let mut failure = (&odd_composites - 
                       &goldbachs_conjecture_table).into_iter()
                                                   .collect::<Vec<_>>();
    failure.sort();
    println!("{:?}", &failure[0]);
}