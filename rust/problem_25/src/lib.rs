/*
The Fibonacci sequence is defined by the recurrence relation:

    Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.

Hence the first 12 terms will be:

    F1 = 1
    F2 = 1
    F3 = 2
    F4 = 3
    F5 = 5
    F6 = 8
    F7 = 13
    F8 = 21
    F9 = 34
    F10 = 55
    F11 = 89
    F12 = 144

The 12th term, F12, is the first term to contain three digits.

What is the index of the first term in the Fibonacci sequence 
to contain 1000 digits?

*/

extern crate num;

use num::BigUint;
use num::One;
use num::Zero;
use num::CheckedAdd;
use std::mem::swap;

#[derive(Debug)]
pub struct FibLengthSeries {
    x: (BigUint, BigUint)
}

impl FibLengthSeries {
    pub fn new() -> FibLengthSeries {
        FibLengthSeries { x: (BigUint::zero(), BigUint::one()) }
    }
}

impl Iterator for FibLengthSeries {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let n = self.x.0.checked_add(&self.x.1);
        if let Some(ref z) = n {
            swap(&mut self.x.0, &mut self.x.1);
            self.x.1 = z.clone();
        }
        let fib_digits = n.unwrap().clone();
        let fib_digit_count = fib_digits.to_string().chars()
                                        .map(|d| d.to_digit(10).unwrap())
                                        .collect::<Vec<u32>>()
                                        .iter()
                                        .count();
        Some(fib_digit_count)
    }
}

pub fn find_length_index(limit: usize) -> usize {
    let x = FibLengthSeries::new().take_while(|n| n < &limit).count() + 1;
    x + 1 // adj since this fib series is 1 + 2 + 3 + 5 ...   
}


#[cfg(test)]
mod tests {
    use find_length_index;

    #[test]
    fn first_three_digit_index() {
        assert_eq!(find_length_index(3), 12);
    }
}
