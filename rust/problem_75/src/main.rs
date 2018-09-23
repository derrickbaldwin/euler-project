#![feature(iterator_step_by)]
extern crate problem_75;
extern crate num_integer;

use problem_75::gcd;
use num_integer::Roots;

fn main() {
    let limit = 1_500_000i64;
    let mut singulars = [0; 1_500_000]; 
    let outer = limit.sqrt() as i64;
    for m in 2i64..outer { 
        for n in 1i64..m {
            if n < m  {
                if (n + m) % 2 == 1 && gcd(m, n ) == 1 {
                    let perim = 2 * (m * m + m * n);
                    let mut k = 1;
                    while k * perim < limit {
                        singulars[(k * perim) as usize] += 1;
                        k += 1 
                    }                   
                }
            }
        }
    } 
    println!("{}", singulars.iter().filter(|&x| x == &1).count());    
}