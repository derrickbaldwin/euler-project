/*
Problem 49:
he arithmetic sequence, 1487, 4817, 8147, in which each 
of the terms increases by 3330, is unusual in two 
ways: (i) each of the three terms are prime, and, 
(ii) each of the 4-digit numbers are permutations of 
one another.

There are no arithmetic sequences made up of three 
1-, 2-, or 3-digit primes, exhibiting this property, 
but there is one other 4-digit increasing sequence.

What 12-digit number do you form by concatenating the 
three terms in this sequence?

Soln: 

Since d = 3330, a vector of primes was set up to hold 
initial quad in sequence from range (1000,3340). 

The next two quads in sequence must be a prime number 
in remaining range (3340-10000) set up in HashSet. 

For each initial quad candidate ... find two sequence 
quads that have same digits (determined by sorting 
quads) and are members of HashSet.

*/

pub fn sort_digit(n: u64) -> String {
    let s = n.to_string();
    let mut digits: Vec<char> = s.chars().collect();
    digits.sort();
    digits.into_iter().collect()
}



