#![feature(i128)]
#![feature(i128_type)]
extern crate problem_65;
extern crate num_traits;
extern crate num_bigint;

use problem_65::big_digit_sum; 
use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::One;

fn main() {
    let mut d: BigUint = One::one();
    let mut n: BigUint = 2.to_biguint().unwrap();
    for i in 2..101 {
        let mut acc: BigUint = d; 
        let mut m: BigUint = if i % 3 == 0 { (2 * (i / 3)).to_biguint().unwrap() } 
                             else { One::one() };
        d = n;
        n = m * &d + acc;
    }
    println!("{}", big_digit_sum(n));
}