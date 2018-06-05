
/*

Problem 34: 

145 is a curious number, as 1! + 4! + 5! = 1 + 24 + 120 = 145.

Find the sum of all numbers which are equal to the sum of the 
factorial of their digits.

Note: as 1! = 1 and 2! = 2 are not sums they are not included.

*/

use std::collections::HashMap;

pub fn factorial(n: i32) -> i32 {
    let mut f = 1;
    for i in 1..(n + 1) {
        f *= i;
    }
    f
}
 
pub fn digit_encoding(f: fn(i32) -> i32) -> HashMap<char, i32> {
    let map: HashMap<char, i32> = 
        [('0', f(0)),
         ('1', f(1)),
         ('2', f(2)),
         ('3', f(3)),
         ('4', f(4)),
         ('5', f(5)),
         ('6', f(6)),
         ('7', f(7)),
         ('8', f(8)),
         ('9', f(9))]
         .iter().cloned().collect();
    map
}

pub fn factorial_digit_sum(digits: Vec<char>) -> i32 {
    let digit_factorials = digit_encoding(factorial);
    digits.iter()
          .map(|x| digit_factorials.get(&x).unwrap())
          .sum()
}


#[cfg(test)]
mod tests {
    use factorial_digit_sum;

    #[test]
    fn factorial_digit_sum_145() {
        assert_eq!(factorial_digit_sum(['1','4','5'].to_vec()), 145);
    }
}
