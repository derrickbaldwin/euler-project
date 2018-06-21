extern crate problem_44;
extern crate itertools;

use problem_44::pentagonal;
use itertools::Itertools;
use std::collections::HashSet;


fn main() {
    let pentagonal_table = (1..3000).map(|x| pentagonal(x))
                                    .collect::<HashSet<_>>();
    'outer: for j in &pentagonal_table {
        'inner: for k in &pentagonal_table {
            if pentagonal_table.contains(&(k - j)) 
                  && pentagonal_table.contains(&(j + k)) {
                println!("({},{}) ->{} {}",j, k, j + k, k - j);
                break 'outer;
            }
        } 
    }   
}