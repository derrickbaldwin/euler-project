extern crate problem_76;

use problem_76::counting_summations;

fn main() {
    let parts = (1..100).collect::<Vec<_>>();
    println!("{}", counting_summations(&parts, 100));
}