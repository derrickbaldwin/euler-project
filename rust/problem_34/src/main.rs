extern crate problem_34;

use problem_34::factorial_digit_sum;

fn main() {
    let lower = 10;
    let upper = 100_000;
    let mut curious_numbers = Vec::new();    
    for x in lower..upper {
        let digits = x.to_string()
                      .chars()
                      .collect::<Vec<char>>();
        if factorial_digit_sum(digits) == x {
            curious_numbers.push(x);
        }
    }
    println!("{}", curious_numbers.iter().sum::<i32>());
}