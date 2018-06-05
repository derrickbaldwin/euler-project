
/*

Problem 33:

The fraction 49/98 is a curious fraction, as an inexperienced 
mathematician in attempting to simplify it may incorrectly 
believe that 49/98 = 4/8, which is correct, is obtained by 
cancelling the 9s.

We shall consider fractions like, 30/50 = 3/5, to be trivial 
examples.

There are exactly four non-trivial examples of this type 
of fraction, less than one in value, and containing two 
digits in the numerator and denominator.

If the product of these four fractions is given in its lowest 
common terms, find the value of the denominator.

*/

pub fn gcd(m: i32, n: i32) -> i32 {
    match m {
        0 => n.abs(),
        _ => gcd(n % m, m)
    }
} 


#[cfg(test)]
mod tests {
    use gcd;

    #[test]
    fn gcd_206_40() {
        assert_eq!(gcd(206, 40), 2);
    }
}
