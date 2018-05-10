extern crate problem_13;

use std::fs::File;
use std::io::prelude::*;
use problem_13::first_10_digits;

fn main() {
    let mut f = File::open("50_digit_numbers.txt").expect("File not found");
    let mut nums = String::new();
    f.read_to_string(&mut nums).expect("Could not read file");
    println!("{}", first_10_digits(nums));
}