extern crate problem_64;

use problem_64::continued_fractions;
use std::f64;


fn main() {
    let odd_period_square_roots = (1..10001).filter(|&x| continued_fractions(x) % 2 != 0)
                                            .count();
    println!("{:?}", odd_period_square_roots);
}