/*

Problem 57:

It is possible to show that the square root of two can be expressed as 
an infinite continued fraction.

√ 2 = 1 + 1/(2 + 1/(2 + 1/(2 + ... ))) = 1.414213...

By expanding this for the first four iterations, we get:

1 + 1/2 = 3/2 = 1.5
1 + 1/(2 + 1/2) = 7/5 = 1.4
1 + 1/(2 + 1/(2 + 1/2)) = 17/12 = 1.41666...
1 + 1/(2 + 1/(2 + 1/(2 + 1/2))) = 41/29 = 1.41379...

The next three expansions are 99/70, 239/169, and 577/408, but the 
eighth expansion, 1393/985, is the first example where the number of digits 
in the numerator exceeds the number of digits in the denominator.

In the first one-thousand expansions, how many fractions contain a numerator 
with more digits than denominator?

*/

extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
use num_traits::FromPrimitive;
use num_bigint::ToBigUint;
use std::mem::replace;

#[derive(Debug)]
pub struct RootConvergence { 
    rc: (BigUint, BigUint)    
}

impl RootConvergence {
    pub fn new() -> RootConvergence {
        RootConvergence { rc: (3.to_biguint().unwrap(), 2.to_biguint().unwrap()) }
    }
}

impl Iterator for RootConvergence {
    type Item = (BigUint, BigUint);
    fn next(&mut self) -> Option<Self::Item> {
        let next_root = { 
            let ref a = self.rc.0;
            let ref b = self.rc.1;
            (a + 2.to_biguint().unwrap() * b, a + b) };
        Some(replace(&mut self.rc, next_root ))
    }
}

pub fn higher_digit_count(x: &BigUint, y: &BigUint) -> bool {
    x.to_string().len() > y.to_string().len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
