extern crate problem_43;
extern crate itertools;

use problem_43::permutate;
use itertools::join;

fn main() {
    // a brute force solution
    let mut digits = [0,1,2,3,4,5,6,7,8,9];
    let mut v = Vec::new();
    for _ in 1..3_265_920+1 {
        permutate(&mut digits);
        if join(&digits[7..10],"").parse::<i32>().unwrap() % 17 == 0 && 
           join(&digits[6..9],"").parse::<i32>().unwrap() % 13  == 0 &&
           join(&digits[5..8],"").parse::<i32>().unwrap() % 11  == 0 &&
           join(&digits[4..7],"").parse::<i32>().unwrap() % 7   == 0 &&
           join(&digits[3..6],"").parse::<i32>().unwrap() % 5   == 0 &&
           join(&digits[2..5],"").parse::<i32>().unwrap() % 3   == 0 &&
           join(&digits[1..4],"").parse::<i32>().unwrap() % 2   == 0 { 
                v.push(join(&digits,"").parse::<i64>().unwrap());
        }
    }
    println!("{:?}", v);
    println!("{}", v.iter().sum::<i64>());
}