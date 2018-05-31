
/*

Problem 26: 

A unit fraction contains 1 in the numerator. The decimal 
representation of the unit fractions with denominators 2 
to 10 are given:

    1/2	= 	0.5
    1/3	= 	0.(3)
    1/4	= 	0.25
    1/5	= 	0.2
    1/6	= 	0.1(6)
    1/7	= 	0.(142857)
    1/8	= 	0.125
    1/9	= 	0.(1)
    1/10	= 	0.1 

Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. 
It can be seen that 1/7 has a 6-digit recurring cycle.

Find the value of d < 1000 for which 1/d contains the longest 
recurring cycle in its decimal fraction part.

soln:
generate list of primes (excluding 3 and 5) up to 1000.
find first (i.e highest) reptend prime from the above reversed list

*/
extern crate primes;

use primes::PrimeSet;

pub fn cycle_length(p: u64) -> u64 {
    let mut a: u64 = 1;
    let mut z: u64 = 0;
    loop {
        a = a * 10 % p;
        z += 1;
        if a == 1 { break; } 
    }
    z
}

pub fn is_reptend_prime(p: u64) -> bool {
    if cycle_length(p) != p - 1 {
        return false
    }
    true
}

pub fn highest_reciprocal_cycle(prime_limit: u64) -> Option<u64> {
    let primes: Vec<u64> = PrimeSet::new().iter()
                                          .skip(3)
                                          .take_while(|n| *n < prime_limit)
                                          .collect::<Vec<u64>>()
                                          .into_iter()
                                          .rev()
                                          .collect();
    
    primes.into_iter().find(|&p| is_reptend_prime(p))    

}


#[cfg(test)]
mod tests {
    use is_reptend_prime;

    #[test]
    fn reptend_prime_7() {
        assert_eq!(is_reptend_prime(7), true);
    }
}
