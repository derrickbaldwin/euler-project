
/*

The primes 3, 7, 109, and 673, are quite remarkable. By taking any two 
primes and concatenating them in any order the result will always be prime. 
For example, taking 7 and 109, both 7109 and 1097 are prime. 

The sum of these four primes, 792, represents the lowest sum for a set 
of four primes with this property.

Find the lowest sum for a set of five primes for which any two primes 
concatenate to produce another prime.

*/


extern crate primal;

use primal::is_prime;

pub fn concat_numbers(x: usize, y: usize) -> usize {
    (format!("{}{}", x.to_string(), y.to_string())).parse().unwrap()
}

pub fn prime_pairs(x: usize, y: usize) -> bool {
    let pair1 = concat_numbers(x, y) as u64;
    let pair2 = concat_numbers(y, x) as u64;
    if is_prime(pair1) && is_prime(pair2) {
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    use prime_pairs;

    #[test]
    fn prime_pairs_7_109() {
        assert_eq!(prime_pairs(7, 109), true);
    }
    
    #[test]
    fn prime_pairs_13_109() {
        assert_eq!(prime_pairs(13, 109), false);
    }

    #[test]
    fn prime_pairs_109_673() {
        assert_eq!(prime_pairs(109, 673), true);
    }
        
}
