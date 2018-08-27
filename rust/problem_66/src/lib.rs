/*
Problem 66:


Consider quadratic Diophantine equations of the form:

x2 – Dy2 = 1

For example, when D=13, the minimal solution in x is 
6492 – 13×1802 = 1.

It can be assumed that there are no solutions in positive 
integers when D is square.

By finding minimal solutions in x for D = {2, 3, 5, 6, 7}, 
we obtain the following:

32 – 2×22 = 1
22 – 3×12 = 1
92 – 5×42 = 1
52 – 6×22 = 1
82 – 7×32 = 1

Hence, by considering minimal solutions in x for D ≤ 7, the 
largest x is obtained when D=5.

Find the value of D ≤ 1000 in minimal solutions of x for which 
the largest value of x is obtained.
*/
extern crate num_traits;
extern crate num_bigint;

use num_bigint::BigUint;
use num_bigint::ToBigUint;
use num_traits::{One, Zero};
use std::f64;
use std::mem;

// Marius Beceanu algorithm in Wikipidia
// modified from problem 64
pub fn continued_fractions(n: i64) -> Vec<u64> {
    let mut mn = 0;
    let mut dn = 1;
    let seed: f64 = n as f64; 
    let mut a0_init = seed.sqrt().floor();
    let mut a0 = seed.sqrt().floor() as i64;
    let mut an_init = seed.sqrt();
    let mut an = seed.sqrt() as i64;
    let mut v = vec!(a0 as u64); 
    
    if an_init - a0_init != 0.0 {
        while an != 2 * a0 {
            mn = (dn * an) - mn;
            dn = (n - (mn * mn)) / dn;
            an = (a0 + mn) / dn;
            v.push(an as u64)
        }
    }
    v.pop();  // remove final an
    return v 
}

pub fn expand_fraction(cf: &Vec<u64>) -> (BigUint, BigUint) {
    let mut num: BigUint = One::one();
    let mut temp = *cf.last().unwrap();
    let mut den: BigUint = temp.to_biguint().unwrap();
    for i in (1..cf.len()-1).rev() {
        num = cf[i] * &den + num;
        mem::swap(&mut den, &mut num);        
    }
    num += cf[0] * &den;
    (num, den)    
}

pub fn is_square(n: i64) -> bool {
    let sq = n as f64;
    sq.sqrt() - sq.sqrt().floor() == 0.0 
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
