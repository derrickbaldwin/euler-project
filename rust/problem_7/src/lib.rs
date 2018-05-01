/*

Problem 7:

By listing the first six prime numbers: 2, 3, 5, 7, 11, 
and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?

*/



fn is_prime(n: u64) -> bool {
    
    let factor_range = (n as f64).sqrt() as u64 + 1;
    for i in 2..factor_range {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn find(n: u64) -> u64 {
    let mut prime_counter = 1;
    let mut idx = 3; //start index at 3
    while prime_counter < n {
        if is_prime(idx) {
            prime_counter += 1;
        }
        idx += 2;
    }
    idx - 2 // adjust index
}

pub fn nth(n: u64) -> Option<u64> {
    match n {
        n if n < 1 => None,
        1          => Some(2),
        _          => Some(find(n)) 
    }
}

#[cfg(test)]
mod tests {
    use nth;

    #[test]
    fn first_prime_number() {
        assert_eq!(nth(1), Some(2));
    }

    #[test]
    fn n_prime_number_for_6th() {
        assert_eq!(nth(6), Some(13));
    }
}
