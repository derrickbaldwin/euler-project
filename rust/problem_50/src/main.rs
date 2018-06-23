extern crate problem_50;
extern crate primes;

use primes::PrimeSet;
use std::collections::HashSet;

fn main() {
    let limit = 1_000_000;
    let mut pt = PrimeSet::new();
    let primes_table = pt.iter()
                         .take_while(|x| x < &limit)
                         .collect::<HashSet<_>>();
    let mut seeded_primes = PrimeSet::new();
    let seeded_boundary = 200;
    let mut max_prime = 0;
    let mut max_runner = 0;
    
    for n in 0..seeded_boundary {
        let mut prime = 0;
        let mut runner = 0;
        for i in max_runner.. {        
            let candidate = seeded_primes.iter()
                                         .skip(n) 
                                         .take(i)
                                         .sum::<u64>();
            if candidate > limit {
                if runner > max_runner {
                    max_runner = runner;
                    max_prime = prime;
                }
                break;
            }
            if primes_table.contains(&candidate) {
                prime = candidate;
                runner = i;
            }     
        }
    }
    println!("{} {}", max_prime, max_runner); 
}