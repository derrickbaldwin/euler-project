/*

Problem 21:

Let d(n) be defined as the sum of proper divisors of 
n (numbers less than n which divide evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b 
are an amicable pair and each of a and b are called 
amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 
5, 10, 11, 20, 22, 44, 55 and 110; 
therefore d(220) = 284. The proper divisors of 284 
are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.

*/

pub fn aliquot_sum(n: u64) -> u64 {
    let mut factors_sum = 0;
    let mut i = 1;
    while i < n {
        if n % i == 0 {
            factors_sum += i
        }
        i += 1; 
    }
    factors_sum  
}

pub fn amicable_numbers(limit: u64) -> Vec<u64> {
    let mut amicable_nums = Vec::new();
    for n in 2..limit {
        let x = aliquot_sum(n);
        if n != x && aliquot_sum(x) == n  {
            amicable_nums.push(n);
        }

    }
    amicable_nums
}

pub fn amicable_numbers_sum(nums: Vec<u64>) -> u64 {
    nums.iter().sum()
} 



#[cfg(test)]
mod tests {
    use aliquot_sum;

    #[test]
    fn proper_divisors_220() {
        assert_eq!(aliquot_sum(220), 284);
    }

    #[test]
    fn proper_divisors_284() {
        assert_eq!(aliquot_sum(284), 220);
    }
}
