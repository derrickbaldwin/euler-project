extern crate problem_2;

use problem_2::Fibonacci;
use problem_2::sum_even_fibs_below_number;

fn main() {
    println!("{:?}", Fibonacci::new().take(10).collect::<Vec<_>>());
    println!("{}", sum_even_fibs_below_number(4_000_000))
}