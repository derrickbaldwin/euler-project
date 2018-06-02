/*

Problem 30:

Surprisingly there are only three numbers that can 
be written as the sum of fourth powers of their digits:

    1634 = 14 + 64 + 34 + 44
    8208 = 84 + 24 + 04 + 84
    9474 = 94 + 44 + 74 + 44

As 1 = 14 is not a sum it is not included.

The sum of these numbers is 1634 + 8208 + 9474 = 19316.

Find the sum of all the numbers that can be written as 
the sum of fifth powers of their digits.

*/


pub fn digit_powers(exp: u32, limit: u32) -> u32 {
    (2..limit).filter(|&x| sum_of_exponent(x, exp) == x)
              .collect::<Vec<u32>>()
              .iter()
              .sum()
} 

fn sum_of_exponent(num: u32, exp: u32) -> u32 {
    num.to_string().chars()
       .map(|n| (n.to_digit(10).unwrap()).pow(exp))                               
       .collect::<Vec<u32>>()
       .iter()
       .sum()
}


#[cfg(test)]
mod tests {
    use sum_of_exponent;
    use digit_powers;

    #[test]
    fn fourth_power_1634() {
        assert_eq!(sum_of_exponent(1634, 4), 1634);
    }

    #[test]
    fn fourth_power_8208() {
        assert_eq!(sum_of_exponent(8208, 4), 8208);
    }

    #[test]
    fn fourth_power_9474() {
        assert_eq!(sum_of_exponent(9474, 4), 9474);
    }

    #[test]
    fn digit_fourth_powers() {
        assert_eq!(digit_powers(4, 9u32.pow(4) * 5), 19316);
    }

}
