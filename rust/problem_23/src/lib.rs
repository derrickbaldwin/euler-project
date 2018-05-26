/*

Problem 23: 

A perfect number is a number for which the sum of its proper divisors 
is exactly equal to the number. For example, the sum of the proper 
divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 
28 is a perfect number.

A number n is called deficient if the sum of its proper divisors 
is less than n and it is called abundant if this sum exceeds n.

As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the 
smallest number that can be written as the sum of two abundant numbers 
is 24. By mathematical analysis, it can be shown that all integers 
greater than 28123 can be written as the sum of two abundant numbers. 
However, this upper limit cannot be reduced any further by analysis even 
though it is known that the greatest number that cannot be expressed 
as the sum of two abundant numbers is less than this limit.

Find the sum of all the positive integers which cannot be written as 
the sum of two abundant numbers.

*/

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient
}

pub fn aliquot_sum(n: usize) -> usize {
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

pub fn classify(num: usize) -> Option<Classification> {
    let result = aliquot_sum(num);
    match num {
        0 => None,
        1 => Some(Classification::Deficient),
        _ => match result {
                result if result < num  => Some(Classification::Deficient),
                result if result == num  => Some(Classification::Perfect),
                result if result > num  => Some(Classification::Abundant),
                _   => None
            }
        }                
}

pub fn non_abundant_sums() -> usize {
    let max: usize = 28123;
    let abundant: Vec<usize> = 
        (1..max+1).filter(|x| classify(*x) == Some(Classification::Abundant))
                  .collect();
    let mut not_abundant_nums = vec![true; max + 1];
    for i in 0..abundant.len() {
        for j in i..abundant.len() {
            let abundant_sum = abundant[i] + abundant[j]; 
            if abundant_sum <= max {
                not_abundant_nums[abundant_sum] = false;
            }
        }
    }
    (1..max + 1).filter(|&x| not_abundant_nums[x]).sum::<usize>()
}


#[cfg(test)]
mod tests {
    use Classification;
    use classify;
    
    #[test]
    fn classify_28() {
        assert_eq!(classify(28), Some(Classification::Perfect));
    }

    #[test]
    fn classify_12() {
        assert_eq!(classify(12), Some(Classification::Abundant));
    }
}
