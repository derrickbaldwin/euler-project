/*

Problem 53:

There are exactly ten ways of selecting three 
from five, 12345:

123, 124, 125, 134, 135, 145, 234, 235, 245, and 345

In combinatorics, we use the notation, 5C3 = 10.

In general, nCr = n! / r!(n−r)!, 
where r ≤ n, n! = n×(n−1)×...×3×2×1, and 0! = 1.

It is not until n = 23, that a value exceeds 
one-million: 23C10 = 1144066.

How many, not necessarily distinct, values of  nCr, 
for 1 ≤ n ≤ 100, are greater than one-million?
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

pub fn combinatorics(n: usize, r: usize) -> BigUint {
    factorial(n) / (factorial(r) * factorial(n - r))
}


#[cfg(test)]
mod tests {
    use combinatorics;
    use num::BigUint;
    use num::FromPrimitive;

    #[test]
    fn combinatorics_23_10() {
        let bu: BigUint = FromPrimitive::from_usize(1_144_066).unwrap();
        assert_eq!(combinatorics(23,10), bu);
    }
}
