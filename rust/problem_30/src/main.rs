extern crate problem_30;

use problem_30::digit_powers;

fn main() {
   println!("{}", digit_powers(4, 9u32.pow(4) * 5)); 
   println!("{}", digit_powers(5, 9u32.pow(5) * 6));
}