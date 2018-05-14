/*

Problem 16

215 = 32768 and the sum of its digits 
is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 21000?

*/

extern crate num_bigint;
extern crate num_traits;

use num_bigint::ToBigUint;

pub fn power_digit_sum(base: usize, exp: usize) -> u32 {
      num_traits::pow(base.to_biguint().unwrap(), exp)
                    .to_string()
                    .chars()
                    .filter_map(|d| d.to_digit(10))
                    .sum()    
}

#[cfg(test)]
mod tests {
    use power_digit_sum;

    #[test]
    fn power_digit_sum_15() {
        assert_eq!(power_digit_sum(2, 15), 26);
    }
}
