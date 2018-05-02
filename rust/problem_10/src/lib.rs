/*

Problem 10:

The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.

*/

pub fn summation_of_primes(n: usize) -> usize {
    let mut sieve = vec![true; n+1];
    sieve[0] = false;
    sieve[1] = false;
    let sqrtlmt = (n as f64).sqrt() as usize + 1; 
    for num in 2..sqrtlmt {
        if sieve[num] {
            let mut multiple = num*num;
            while multiple <= n {
                sieve[multiple] = false;
                multiple += num;
            }
        }
    }
    sieve.iter()
         .enumerate()
         .filter_map(|(p, &is_prime)| if is_prime {Some(p)} else {None})
         .fold(0,|acc, p| acc + p)
}

#[cfg(test)]
mod tests {
    use summation_of_primes;

    #[test]
    fn prime_summation_10() {
        assert_eq!(summation_of_primes(10), 17);  
    }
}
