extern crate problem_66;
extern crate num_traits;
extern crate num_bigint;


use problem_66::continued_fractions;
use problem_66::is_square;
use problem_66::expand_fraction;
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::{One, Zero};

fn main() {
    println!("{:?}", "Problem 66");
    let d = (2i64..1001).filter(|&x| !is_square(x))
                        .collect::<Vec<i64>>();
         
    let mut max: BigUint = Zero::zero();
    let mut d_idx = 0;
    for i in d {
        let cf = continued_fractions(i);
        let x = expand_fraction(&cf);
        let mut u: BigUint = x.0.to_biguint().unwrap();
        let mut v: BigUint = x.1.to_biguint().unwrap();
        if cf.len() % 2 != 0 {
            let o: BigUint = One::one();
            u = 2.to_biguint().unwrap() * &u * &u + o;
            v = 2.to_biguint().unwrap() * &u * v; 
        } 
        if &u > &max {
            max = u;
            d_idx = i;
        }
    }
    println!("{} {}", d_idx, max);
}