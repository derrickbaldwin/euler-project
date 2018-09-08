extern crate problem_73;

use problem_73::fractional_range_count;

fn main() {
    println!("{}", fractional_range_count((1,3), (1,2), 12_000));
}