extern crate problem_67;

use std::fs::File;
use std::io::prelude::*;
use problem_67::max_path;

fn main() {
  
    let mut f = File::open("p067_triangle.txt").expect("File not found");
    let mut nums = String::new();
    f.read_to_string(&mut nums).expect("Could not read file");

    let triangular: Vec<Vec<i32>> = nums.lines()
                                       .map(|row| {
                                            row.split_whitespace()
                                               .filter_map(|d| d.parse().ok())
                                               .collect()
                                       }).collect();
    
    println!("{}", max_path(&triangular));    
}