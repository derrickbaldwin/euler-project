extern crate problem_21;

use problem_21::amicable_numbers;
use problem_21::amicable_numbers_sum;

fn main() {
    println!("{}", amicable_numbers_sum(amicable_numbers(10_000)));

}
