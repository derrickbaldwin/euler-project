#![feature(slice_rotate)]

extern crate problem_35;
extern crate primes;
extern crate regex;

use problem_35::is_circular;
use primes::PrimeSet;
use regex::Regex;

fn main() {
    let single_circular_primes = vec![2, 3, 5, 7];
    let re = Regex::new("^[1379]{2,6}$").unwrap();
    let mut primes = PrimeSet::new();
    let nums = primes.iter()
                     .take_while(|&p| p < 1_000_000)
                     .map(|p| p.to_string()) 
                     .filter(|p| re.is_match(p))
                     .collect::<Vec<_>>();
  
    let circular_primes = nums.iter()
                              .filter(|n| is_circular(n))
                              .count(); 
    
    // add the 4 single digit primes
    println!("{}", single_circular_primes.len() + circular_primes);
}