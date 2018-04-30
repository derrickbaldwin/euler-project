extern crate problem_6;

use problem_6::algebraic_sum_square_difference;
use problem_6::iterative_sum_square_difference;


fn main() {
    println!("{}", algebraic_sum_square_difference(100));
    assert_eq!(algebraic_sum_square_difference(100), 
                    iterative_sum_square_difference(100));   
}