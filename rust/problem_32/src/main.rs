extern crate problem_32;

use std::collections::HashSet;
use problem_32::is_product_pandigital;

fn main() {
    let mut set = HashSet::new();
    for multiplicand in 1..100 {
        for multiplier in 1..10_000 {
            let product  = multiplicand * multiplier;
            let digits = format!("{}{}{}", multiplicand,multiplier,product);
            if digits.len() == 9 && is_product_pandigital(&digits) {
                set.insert(product);
            }
            
        }
    }
    println!("{:?}", set);
    println!("{:?}", set.iter().sum::<u64>());
}

