extern crate problem_20;

use problem_20::factorial;
use problem_20::sum_of_factorial_digits;

fn main() {
    println!("{}", sum_of_factorial_digits(factorial(100)));
}