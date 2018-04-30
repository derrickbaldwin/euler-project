/*

Problem 3:

The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the 
number 600851475143 ?


*/

pub fn largest_prime_factor(n: u64) -> u64 {
    let mut f = 1;
    let mut z = n;
    
    while z > 1 {
        f += 1;
        while z % f == 0 {
            z = z / f;        
        }
    }
    f
}


#[cfg(test)]
mod tests {
    use largest_prime_factor;

    #[test]
    fn it_works() {
        assert_eq!(largest_prime_factor(13_195), 29);
    }
}
