extern crate problem_51;
extern crate primes;

use problem_51::digit_count;
use problem_51::family;
use primes::PrimeSet;
use std::collections::HashMap;


fn main() {
    // create a 6-digit prime interval
    let mut pset = PrimeSet::new();
    let (lower, _) = pset.find(100_000);
    let (upper, _) = pset.find(1_000_000);
    let six_digit_primes = pset.iter()
                               .skip(lower)
                               .take(upper- lower + 1)
                               .collect::<Vec<_>>();    
    
    // filter primes into map that has exactly 3 target digits (0,1, or 2)
    let mut map = HashMap::new();
    for d in &six_digit_primes {
        let s = d.to_string();
        let digits = s.chars().collect::<Vec<_>>();
        if digit_count(&digits, '0') == 3 && digits[5] != '0' {
            map.entry("0").or_insert(Vec::new()).push(s);
            continue;            
        }
        if digit_count(&digits, '1') == 3 && digits[5] != '1' {
            map.entry("1").or_insert(Vec::new()).push(s);
            continue;
        } 
        if digit_count(&digits, '2') == 3 && digits[5] != '2' {
            map.entry("2").or_insert(Vec::new()).push(s);
        }
    }

    // create prime families by digit replacement 
    let mut prime_families = Vec::new();
    for (k,v) in map {
        for primes in v {
            prime_families.push(family(k, primes));
        }
    }

    // find prime family with 8 members and display smallest member
    let target_family = prime_families.iter().find(|x| x.len() == 8);
    println!("{:?}", target_family.unwrap()[0]);     
}