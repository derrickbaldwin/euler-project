/*
Problem 51:

By replacing the 1st digit of the 2-digit number *3, 
it turns out that six of the nine possible values: 
13, 23, 43, 53, 73, and 83, are all prime.

By replacing the 3rd and 4th digits of 56**3 with the 
same digit, this 5-digit number is the first example 
having seven primes among the ten generated numbers, 
yielding the family: 56003, 56113, 56333, 56443, 56663, 
56773, and 56993. Consequently 56003, being the first 
member of this family, is the smallest prime with this property.

Find the smallest prime which, by replacing part of the number 
(not necessarily adjacent digits) with the same digit, 
is part of an eight prime value family.

*/
extern crate primes;

use primes::is_prime;

pub fn digit_count(ds: &[char], target: char) -> u64 {
    ds.iter().filter(|&c| c == &target).count() as u64
}

pub fn family(replacement_digit: &str, prime: String) -> Vec<u64>  {
    let mut family_members = Vec::new();
    for n in 1..10 { 
        let candidate = (prime.replacen(replacement_digit, &n.to_string(), 3))
                              .parse::<u64>().unwrap();
        if is_prime(candidate) {
            family_members.push(candidate);
        }
    }
    family_members
}


#[cfg(test)]
mod tests {
    use family;

    #[test]
    fn family_of_6() {
        // note the 0s are dropped from the example test case shown in 
        // problem description.
        assert_eq!(family("1","56113".to_string()), 
                       vec![56113, 56333, 56443, 56663, 56773, 56993]);
    }
}
