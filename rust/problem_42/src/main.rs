extern crate problem_42;


use std::fs::File;
use std::io::prelude::*;

use problem_42::score;
use std::collections::HashSet;
use std::collections::HashMap;


fn main() {
    // load the file
    let mut f = File::open("p042_words.txt").expect("File not found");
    let mut text = String::new();
    f.read_to_string(&mut text).expect("Could not read file");
    
    // create list of names
    let names: Vec<&str> = text.split(",")
                               .map(|s| s.trim().trim_matches('\"'))
                               .collect();
    
    // create membership lookup table    
    let triangular_numbers = (0..25).map(|x| (x * (x + 1))/ 2)
                                    .collect::<HashSet<_>>(); 
    
    // insert eligible numbers in hash map
    let mut map = HashMap::new();
    for n in &names {
        let z = score(n);
        if triangular_numbers.contains(&z) {
            *map.entry(z).or_insert(0) += 1;
        }
    }
    
    // sum map values
    println!("{}", map.values().sum::<u32>());
}