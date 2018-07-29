/*

Problem 55:

If we take 47, reverse and add, 47 + 74 = 121, which is palindromic.

Not all numbers produce palindromes so quickly. For example,

349 + 943 = 1292,
1292 + 2921 = 4213
4213 + 3124 = 7337

That is, 349 took three iterations to arrive at a palindrome.

Although no one has proved it yet, it is thought that some numbers, like 196, never produce a palindrome. A number that never forms a palindrome through the reverse and add process is called a Lychrel number. Due to the theoretical nature of these numbers, and for the purpose of this problem, we shall assume that a number is Lychrel until proven otherwise. In addition you are given that for every number below ten-thousand, it will either (i) become a palindrome in less than fifty iterations, or, (ii) no one, with all the computing power that exists, has managed so far to map it to a palindrome. In fact, 10677 is the first number to be shown to require over fifty iterations before producing a palindrome: 4668731596684224866951378664 (53 iterations, 28-digits).

Surprisingly, there are palindromic numbers that are themselves Lychrel numbers; the first example is 4994.

How many Lychrel numbers are there below ten-thousand?

NOTE: Wording was modified slightly on 24 April 2007 to emphasise the theoretical nature of Lychrel numbers.
*/

extern crate num_bigint;
extern crate num_traits;

use std::str::FromStr;
use num_bigint::BigUint;
use num_traits::cast::FromPrimitive;

pub fn is_lychrel(num: u64) -> bool {
    let num: BigUint = FromPrimitive::from_u64(num).unwrap();
    let mut s = num;
    let mut repeat = 0u64;
    loop {
        if repeat > 50 { return true; }
        s += reverse(&s);
        repeat += 1;
        if s == reverse(&s) { return false; }
    }    
}

pub fn reverse(num: &BigUint) -> BigUint {
    let big = num.to_string()
                 .chars()
                 .rev()
                 .collect::<String>();
    
    FromStr::from_str(&big).unwrap() 
}


#[cfg(test)]
mod tests {
    use is_lychrel;

    #[test]
    fn lychrel_47() {
        assert_eq!(is_lychrel(47), false);
    }
    
    #[test]
    fn lychrel_196() {
        assert_eq!(is_lychrel(196), true);
    }
    
    #[test]
    fn lychrel_349() {
        assert_eq!(is_lychrel(349), false);
    }

    #[test]
    fn lychrel_4994() {
        assert_eq!(is_lychrel(4994), true);
    }

    #[test]
    // > 50 iterations, therfore false
    fn lychrel_10667() {
        assert_eq!(is_lychrel(10667), false);
    }

}
