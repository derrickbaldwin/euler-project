/*

Problem 36:

The decimal number, 585 = 10010010012 (binary), is 
palindromic in both bases.

Find the sum of all numbers, less than one million, 
which are palindromic in base 10 and base 2.

(Please note that the palindromic number, in either 
base, may not include leading zeros.)

*/

pub fn is_palindrome(num: &str) -> bool {
    let mid = num.len()/2;
    num.chars()
       .take(mid)
       .eq(num.chars()
              .rev()
              .take(mid))
}

pub fn double_base_palindromes(limit: i32) -> i32 {
    (1..limit).filter(|x| x % 2 != 0 &&
                          is_palindrome(&x.to_string()) && 
                          is_palindrome(&format!("{:b}", x))).sum()
              
}

#[cfg(test)]
mod tests {
    use is_palindrome;

    #[test]
    fn is_palindrome_585_base10() {
        assert_eq!(is_palindrome("585"), true);
    }

    #[test]
    fn is_palindrome_585_base2() {
        assert_eq!(is_palindrome("1001001001"), true);
    }
}
