/*
Problem 78:


Let p(n) represent the number of different ways in which n coins can 
be separated into piles. For example, five coins can be separated 
into piles in exactly seven different ways, so p(5)=7.

      OOOOO
     OOOO   O
    OOO   OO
   OOO   O   O
   OO   OO   O
  OO   O   O   O
 O   O   O   O   O

Find the least value of n for which p(n) is divisible by one million.
*/
#[macro_use]
extern crate interleave;

use interleave::{IterList, MultiIter};
 
pub fn generalized_pentagonal_numbers(n: i64) -> Vec<usize> {
    interleave!(
        (1..n).map(|k| pent_positive(k)).collect::<Vec<_>>().into_iter(),
        (1..n).map(|k| pent_negative(k)).collect::<Vec<_>>().into_iter()
    ).collect::<Vec<_>>()
} 

fn pent_positive(x: i64) -> usize {
    let r = x * (3 * x - 1) / 2; 
    r as usize
}

fn pent_negative(x: i64) -> usize {
    let r =  - x * (3 *  - x - 1) / 2; 
    r as usize
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
