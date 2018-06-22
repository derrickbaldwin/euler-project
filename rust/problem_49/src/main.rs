extern crate problem_49;
extern crate primes;
extern crate itertools;

use problem_49::sort_digit;
use primes::is_prime;
use std::collections::HashSet;


fn main() {
    let initial_primes = (1000u64..3340).filter(|&x| is_prime(x))
                                        .collect::<Vec<_>>();
    let eligible_primes = (3340u64..10000).filter(|&x| is_prime(x))
                                          .collect::<HashSet<_>>();
    let d = 3330;
    for x in initial_primes {
        let a = x + d;
        let b = x + 2 * d;
        if eligible_primes.contains(&a) && eligible_primes.contains(&b) {
            let s1 = sort_digit(x);
            if s1.eq(&sort_digit(a)) && s1.eq(&sort_digit(b)) {
                println!("{}{}{}", x, a, b);
            }
        }
    }
}