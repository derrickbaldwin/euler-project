
/*
Problem 77:

It is possible to write ten as the sum of primes in exactly 
five different ways:

7 + 3
5 + 5
5 + 3 + 2
3 + 3 + 2 + 2
2 + 2 + 2 + 2 + 2

What is the first value which can be written as the sum of 
primes in over five thousand different ways?
*/

extern crate primes;


pub fn counting_summations(parts: &[usize], sum: usize) -> usize {
    let size = sum + 1;
    let mut combo = vec![0; size];
    combo[0] = 1;
    for &part in parts {
        for amount in part..size {
            combo[amount] += combo[amount - part];
        }
    }
    combo[sum]
}


#[cfg(test)]
mod tests {
    use counting_summations;

    #[test]
    fn primes_for_10() {
        assert_eq!(counting_summations(&[2, 3, 5, 7], 10), 5);
    }
}
