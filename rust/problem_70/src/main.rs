#[macro_use] extern crate itertools;
extern crate problem_70;
extern crate primes;

use problem_70::*;
use primes::PrimeSet;

fn main() {
    let mut pset = PrimeSet::new();
    let ps = pset.iter().skip(300).take(375).collect::<Vec<_>>();
    
    let limit = 10_000_000;
    let mut ratio: f64 = 0.0;
    let mut best_n = 1;
    let mut best_ratio: f64 = 10_000.0;
    for p in &ps {
        for q in &ps[1..] { 
            let n = p * q;
            if n > limit {
                break; 
            }
            let phi = (p - 1) * (q - 1);
            ratio = n as f64 / phi as f64;
            if is_perm(n, phi) && best_ratio > ratio {
                best_n = n;
                best_ratio = ratio;
            }
        }        
    }
    println!("{}" , best_n);
}


