extern crate problem_74;

use problem_74::chain;
use std::collections::HashMap;

fn main() {
    let mut termination: HashMap<u64,u64> = 
        [(0,1),
        (1,1),
        (2,1),
        (145,1),
        (169,3),
        (871,2),
        (872,2),
        (1454,3),
        (40585,1),
        (45361,2),
        (45362,2),
        (363601,3)]
        .iter().cloned().collect();
    
    println!("{}", (2..1_000_001).filter(|&x| chain(x, &mut termination) == 60 ).count());
}
    
    
