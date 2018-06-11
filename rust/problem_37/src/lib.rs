/*

Problem 37:

The number 3797 has an interesting property. Being prime 
itself, it is possible to continuously remove digits from 
left to right, and remain prime at each stage: 3797, 797, 
97, and 7. 

Similarly we can work from right to left: 3797, 379, 
37, and 3.

Find the sum of the only eleven primes that are both 
truncatable from left to right and right to left.

NOTE: 2, 3, 5, and 7 are not considered to be truncatable 
primes.

*/

extern crate primes;
extern crate itertools;

use primes::is_prime;
use itertools::join;

pub fn is_right_truncatable(digits: &[char], l: usize) -> bool {
    let mut v = Vec::new();
    for n in 0..l-1 {
        v.push(join(&digits[0..l - n - 1], "")
         .parse::<u64>()
         .unwrap());        
    }
    is_all_prime(&v)
}


pub fn is_left_truncatable(digits: &[char], l: usize) -> bool {
    let mut v = Vec::new();
    for n in 0..l-1{
        v.push(join(&digits[n+1..l], "")
         .parse::<u64>()
         .unwrap());        
    }
    is_all_prime(&v)
}


fn is_all_prime(v: &[u64]) -> bool {
   v.iter().all(|&x| is_prime(x)) 
}

pub fn is_two_sided_prime(num: u64) -> bool {
    let digits = num.to_string().chars().collect::<Vec<_>>();
    let length = digits.len();
    is_right_truncatable(&digits, length) && is_left_truncatable(&digits, length)
}

#[cfg(test)]
mod tests {
    use is_two_sided_prime;
    
    #[test]
    fn is_two_sided_prime_23() {
        assert_eq!(is_two_sided_prime(23), true);
    }

    #[test]
    fn is_two_sided_prime_3797() {
        assert_eq!(is_two_sided_prime(3797), true);
    }

    #[test]
    fn is_two_sided_prime_67() {
        assert_eq!(is_two_sided_prime(67), false);
    }

    #[test]
    fn is_two_sided_prime_233() {
        assert_eq!(is_two_sided_prime(233), false);
    }

    #[test]
    fn is_two_sided_prime_739397() {
        assert_eq!(is_two_sided_prime(739397), true);
    }


}
