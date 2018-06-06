/*

Problem 35:

The number, 197, is called a circular prime because all rotations of 
the digits: 197, 971, and 719, are themselves prime.

There are thirteen such primes below 100: 2, 3, 5, 7, 11, 
13, 17, 31, 37, 71, 73, 79, and 97.

How many circular primes are there below one million?

*/
#![feature(slice_rotate)]

extern crate primes;

use primes::is_prime;

pub fn is_circular(primes: &str) -> bool {
    assert!(primes.len() > 1);
    let mut rotation_candidates: Vec<String> = Vec::new();
    let mut z = primes.chars().collect::<Vec<_>>();
    for _ in 0..z.len()-1 {
        z.rotate_right(1);
        let c = z.clone(); 
        rotation_candidates.push(c.into_iter().collect());
    }
    rotation_candidates.iter()
        .all( |ref x| is_prime(x.parse::<u64>()
        .unwrap()))
}


#[cfg(test)]
mod tests {
    use is_circular;

    #[test]
    fn circular_prime_197() {
        assert_eq!(is_circular("197"), true);
    }
}
