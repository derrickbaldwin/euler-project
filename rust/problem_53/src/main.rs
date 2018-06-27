extern crate problem_53;
extern crate num;


use problem_53::combinatorics;
use num::{BigUint, FromPrimitive};

fn main() {
    let upper = 100;
    let ceiling = 1_000_000;
    let mut counter = 0;
    let ceiling: BigUint = FromPrimitive::from_usize(ceiling).unwrap(); 
    for n in 23..upper+1 {
        for r in 1..n {
            if combinatorics(n , r) > ceiling {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}