/*

Problem 4:

A palindromic number reads the same both ways. The largest 
palindrome made from the product of two 2-digit numbers 
is 9009 = 91 × 99.

Find the largest palindrome made from the product of 
two 3-digit numbers.

*/


fn is_number_palindrome(num: u64) -> bool {
    let mut rev = 0;
    let mut n = num;

    while n > 0 {
        rev = rev * 10 + n % 10;
        n /= 10;
    }
    num == rev
}

pub fn largest_palindrome_product(digits: u32) -> Option<u64> {
    let low: u64 = 10u64.pow(digits - 1);
    let high: u64 = low * 10;
    let v: Vec<u64> = (low..high).rev().collect();
    if digits < 2 {
        return None;
    }
    let mut acc_max_palindrome = 0;
    'outside: for i in &v {
        'inside: for j in &v {
            let mut temp = i * j;
            if is_number_palindrome(temp) && temp > acc_max_palindrome {
                acc_max_palindrome = temp;
            }
        }
    }
    Some(acc_max_palindrome)
}


#[cfg(test)]
mod tests {

    use is_number_palindrome;
    use largest_palindrome_product;

    #[test]
    fn palindrone_1001() {
        assert_eq!(is_number_palindrome(1001), true);
    }

    #[test]
    fn not_palindrone_551233() {
        assert_eq!(is_number_palindrome(551233), false);
    }

    #[test]
    fn largest_2_digit_palindrome() {
        assert_eq!(largest_palindrome_product(2).unwrap(), 9009);
    }
}
