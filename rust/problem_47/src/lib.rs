/*

Problem 47:

The first two consecutive numbers to have two distinct 
prime factors are:

14 = 2 × 7
15 = 3 × 5

The first three consecutive numbers to have three 
distinct prime factors are:

644 = 2² × 7 × 23
645 = 3 × 5 × 43
646 = 2 × 17 × 19.

Find the first four consecutive integers to have four distinct 
prime factors each. 

What is the first of these numbers?

Soln approach:
A bit heavy on abstraction, ... (did not optimize after 
this first solution). 

-create vector of sliding tuple windows from unique 
prime factors lengths 

-search for first 4-tuple that matches (4, 4, 4, 4) 

*/

use std::collections::HashSet;

pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut f = 1;
    let mut z = n;
    
    while z > 1 {
        f += 1;
        while z % f == 0 {
            v.push(f);
            z = z / f;        
        }
    }
    v
}

pub fn unique_prime_factors_length(n: u64) -> u64 {
    let distinct_prime_factors: HashSet<u64> = 
                prime_factors(n).iter().cloned().collect();
    distinct_prime_factors.len() as u64
}

pub fn all_fours(w: (u64,u64,u64,u64)) -> bool {
    match w {
        (4,4,4,4) => true,
        _         => false 
    }
}


#[cfg(test)]
mod tests {
    use unique_prime_factors_length;

    #[test]
    fn prime_factor_length_15() {
        assert_eq!(unique_prime_factors_length(15), 2);
    }
    #[test]
    fn prime_factor_length_644() {
        assert_eq!(unique_prime_factors_length(644), 3);
    }
    #[test]
    fn prime_factor_length_646() {
        assert_eq!(unique_prime_factors_length(646), 3);
    }
}
