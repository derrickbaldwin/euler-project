extern crate problem_55;


use problem_55::is_lychrel;
use problem_55::reverse;

fn main() {
    println!("{}",(1..10001).filter(|&n| is_lychrel(n)).count());
}

