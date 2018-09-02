extern crate problem_68;
extern crate permutohedron;
extern crate itertools;

use problem_68::*;
use permutohedron::Heap;
use std::collections::BinaryHeap;

fn main() {
    // bruteforce: set up 9! permutations
    let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let heap = Heap::new(&mut data);
    let mut perms = Vec::new();
    for data in heap {
        perms.push(data.clone());
    }
    // iterate permutations & place ngon-5s in btree and pop the max
    let mut ngon5s = BinaryHeap::new(); 
    for perm in perms {
        let mut n = Ngon5::new(perm); 
        if n.is_magic() {
            ngon5s.push(n.value());
        }
    }
    println!("{}", ngon5s.pop().unwrap())
}
