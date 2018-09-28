extern crate problem_79;

use problem_79::position_swap;

fn main() {
    let data = [ 319, 680, 180, 690, 129, 620, 762, 689, 762, 318,
                 368, 710, 720, 710, 629, 168, 160, 689, 716, 731,
                 736, 729, 316, 729, 729, 710, 769, 290, 719, 680,
                 318, 389, 162, 289, 162, 718, 729, 319, 790, 680,
                 890, 362, 319, 760, 316, 729, 380, 319, 728, 716,
               ];
        
    let mut slots = vec![0, 1, 2, 3, 6, 7, 8, 9];

    let mut nums = Vec::new();
    for num in data.iter() {
        nums.push(num.to_string()
                     .chars()
                     .map(|d| d.to_digit(10).unwrap())
                     .collect::<Vec<_>>());
    } 
    
    for n in nums {
        position_swap(n[0], n[1], &mut slots);
        position_swap(n[1], n[2], &mut slots);
    }
  
    println!("{:?}", slots);    
}