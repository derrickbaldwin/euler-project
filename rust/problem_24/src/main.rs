extern crate problem_24;

use problem_24::permutate;

fn main() {
    let mut digits = [0,1,2,3,4,5,6,7,8,9];   
    for _ in 1..1_000_000 {
        permutate(&mut digits);
    }
    println!("{:?}", digits);
}