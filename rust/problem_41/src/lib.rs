/*
Problem 41:

We shall say that an n-digit number is pandigital if 
it makes use of all the digits 1 to n exactly once. 
For example, 2143 is a 4-digit pandigital and is also prime.

What is the largest n-digit pandigital prime that exists?

...

soln approach:
Push 4-digit & 7-digit permutations onto binary heap. 
Then pop off the heap (next max) until prime is found.

*/

extern crate primes;
extern crate permutohedron;
extern crate itertools;

use permutohedron::Heap;
use itertools::join;

pub fn pandigitals(n: u64) -> Vec<u64> {
    let mut data: Vec<u64> = (1..n+1).collect();
    let heap = Heap::new(&mut data);
    let mut permutations = Vec::new();
    for data in heap {
        permutations.push(data.clone());
    }
    let mut v = Vec::new();
    for x in permutations {
        v.push(join(x, "").parse::<u64>().unwrap());
    }
    v
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
