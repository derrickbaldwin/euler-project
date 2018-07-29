extern crate problem_56;

use problem_56::power_digit_sum;

fn main() {
    let mut max_sum = 0;
    for i in 1..100 {
        for j in 1..100 {
            let pd = power_digit_sum(i, j);
            if pd > max_sum {
                max_sum = pd;
            }
        }
    }
    println!("{}", max_sum);
    
}

