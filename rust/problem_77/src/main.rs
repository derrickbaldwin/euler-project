extern crate problem_77;
extern crate primes;

use primes::PrimeSet; 
use problem_77::counting_summations;


fn main() {
    const N: usize = 1000;
    
    let limit = 5000;
    let mut primes = [0usize; N];
    
    let mut pset = PrimeSet::new();
    for (i, p) in pset.iter().enumerate().take(N) {
         primes[i] = p as usize;
    }
    println!("{}", (1..).skip_while(|&n| counting_summations(&primes, n) <= limit).next().unwrap());    
}