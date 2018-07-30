extern crate problem_57;

use problem_57::RootConvergence;
use problem_57::higher_digit_count;

fn main() {
    let expansions = 1000;
    let count = RootConvergence::new().take(expansions)
                                      .filter(|&(ref x, ref y)| higher_digit_count(x, y))
                                      .count();
    println!("{}", count);
}