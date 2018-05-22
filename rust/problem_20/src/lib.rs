/*

Problem 20:

n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 
3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!

*/
extern crate num;

use num::{BigUint, One, FromPrimitive};

pub fn factorial(n: usize) -> BigUint {
    let mut f: BigUint = One::one();
    for i in 1..(n+1) {
        let bu: BigUint = FromPrimitive::from_usize(i).unwrap();
        f = f * bu;
    }
    f
}

pub fn sum_of_factorial_digits(bu: BigUint) -> u32 {
    bu.to_string()
      .chars()
      .filter_map(|d| d.to_digit(10))
      .sum() 
}


#[cfg(test)]
mod tests {
    use factorial;
    use sum_of_factorial_digits;

    #[test]
    fn it_works() {
        assert_eq!(sum_of_factorial_digits(factorial(10)), 27);
    }
}
