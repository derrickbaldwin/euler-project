extern crate problem_37;
extern crate primes;

use primes::PrimeSet;
use problem_37::is_two_sided_prime;


fn main() {
    let mut primes = PrimeSet::new();
    let result = primes.iter()
                       .skip(4) 
                       .take_while(|&n| n < 1_000_000)
                       .filter(|&p| is_two_sided_prime(p))
                       .collect::<Vec<_>>();
    println!("{:?}", result);                   
    println!("{:?}", result.iter().count());                   
    println!("{:?}", result.iter().sum::<u64>());
}

