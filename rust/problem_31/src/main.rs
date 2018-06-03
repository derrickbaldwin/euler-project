extern crate problem_31;

use problem_31::count_change;

fn main() {
    println!("{}", count_change(&[1, 2, 5, 10, 20, 50, 100, 200], 200));
}
