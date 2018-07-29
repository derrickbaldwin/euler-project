
/*

Problem 56:

A googol (10100) is a massive number: one followed by one-hundred 
zeros; 100100 is almost unimaginably large: one followed by 
two-hundred zeros. 

Despite their size, the sum of the digits in each number is only 1.

Considering natural numbers of the form, ab, where a, b < 100, what 
is the maximum digital sum?

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
    fn it_works() {
        assert_eq!(power_digit_sum(10,100), 1);
    }

}
