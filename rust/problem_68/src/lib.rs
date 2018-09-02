/*

The great German Philosopher Immanual Kant once said "If man makes himself 
a worm, he must not complain when he is trodden on"

Problem 68:

Consider the following "magic" 3-gon ring, filled with the 
numbers 1 to 6, and each line adding to nine.

Working clockwise, and starting from the group of three 
with the numerically lowest external node 
(4,3,2 in this example), each solution can be described 
uniquely. For example, the above solution can be described 
by the set: 4,3,2; 6,2,1; 5,1,3.

It is possible to complete the ring with four different 
totals: 9, 10, 11, and 12. There are eight solutions in total.

Total	Solution Set
9	4,2,3; 5,3,1; 6,1,2
9	4,3,2; 6,2,1; 5,1,3
10	2,3,5; 4,5,1; 6,1,3
10	2,5,3; 6,3,1; 4,1,5
11	1,4,6; 3,6,2; 5,2,4
11	1,6,4; 5,4,2; 3,2,6
12	1,5,6; 2,6,4; 3,4,5
12	1,6,5; 3,5,4; 2,4,6

By concatenating each group it is possible to form 9-digit 
strings; the maximum string for a 3-gon ring is 432621513.

Using the numbers 1 to 10, and depending on arrangements, 
it is possible to form 16- and 17-digit strings. What is 
the maximum 16-digit string for a "magic" 5-gon ring?

SOLUTION:
1) Uses bruteforce permutations (9!) on Ngon5 objects. 

2) Ngon5 object holds a vector of 5 Lines translated from 
outer/inner indexed structure anchored with a fixed 10. 

3) Validates magical Ngon5s and enforces clockwise number 
formation with circular vector. 

4) Ngon5s are placed in btree and the max is popped off

*/
extern crate itertools;

use std::fmt;
use itertools::max;
use itertools::min;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line {
    pub a: u64,
    pub b: u64,
    pub c: u64,
}

impl Line {
    // outer ring element is value of line
    pub fn val(self) -> u64 {
        self.a
    }
    // sum of line
    pub fn sum(self) -> u64 {
        self.a + self.b + self.c
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.a, self.b, self.c)
    }
}

pub struct Ngon5 { 
    ngon: Vec<Line>,
}

impl Ngon5 {
    // indexed inner/outer structure of lines anchored with 10 
    pub fn new(x: Vec<u64>) -> Ngon5 {
        Ngon5 { ngon: vec![ Line { a: x[5], b: x[0], c: x[1] },
                            Line { a: x[6], b: x[1], c: x[2] },
                            Line { a: x[7], b: x[2], c: x[3] },
                            Line { a: x[8], b: x[3], c: x[4] },
                            Line { a: 10,   b: x[4], c: x[0] },  // anchored 10!
                          ]}
    }
   
    // are all lines equal
    pub fn is_magic(&mut self) -> bool {
        let line_sums = vec![ self.ngon[0].sum(), 
                              self.ngon[1].sum(), 
                              self.ngon[2].sum(), 
                              self.ngon[3].sum(), 
                              self.ngon[4].sum()
                            ];
        
        line_sums.iter().all_equal()
    }

    // helper: get starting line index
    fn starting_idx(&mut self) -> u64 {
        let mut vals = vec![ self.ngon[0].val(), 
                             self.ngon[1].val(), 
                             self.ngon[2].val(), 
                             self.ngon[3].val(), 
                             self.ngon[4].val()
                        ];
        *vals.iter().min().unwrap()
    }

    // circular vector to build the clockwise number 
    pub fn value(mut self) -> u64 {
        let seed: u64 =  self.starting_idx();
        let mut current_idx = self.ngon.iter()
                                       .position(|x| x.val() == seed)
                                       .unwrap();
        let mut num = String::new();
        for _ in 0..5 {
            let x = self.ngon[current_idx];
            num += &x.to_string();
            current_idx += 1;
            current_idx %= 5;
        }
        num.parse::<u64>().unwrap()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
