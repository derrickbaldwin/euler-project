extern crate problem_62;
extern crate itertools;

use problem_62::*;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    println!("{}", "Problem 62");
    
    let cubes = (100..10_000).map(|x| { let s = cube(x).to_string(); (s.len(), s) })
                             .collect::<Vec<_>>();
    let cubes_table = cubes.into_iter().into_group_map();
        
    for i in 7..13 {
        let candidates = &cubes_table[&i];
        let mut hash = HashMap::new();
        for val in candidates {
            let key = sorted_cube(val);
            hash.entry(key).or_insert(Vec::new()).push(val);
        }
        for x in hash.values() {
            if x.iter().len() == 5 {
                println!("{} -> {:?}", i, x);
            }
        }
    } 
  
}