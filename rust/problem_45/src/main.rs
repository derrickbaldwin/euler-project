extern crate problem_45;

use problem_45::hexagonal;
use problem_45::pentagonal;
use problem_45::triangle;
use std::collections::HashSet;

fn main() {
    let limit = 100_000; 
    let triangles = (286..limit).map(|x| triangle(x)).collect::<HashSet<_>>();
    let pentagonals = (166..limit).map(|x| pentagonal(x)).collect::<HashSet<_>>();
    let hexagonals = (144..limit).map(|x| hexagonal(x)).collect::<HashSet<_>>();
    println!("{:?}", &(&triangles & &pentagonals) & &hexagonals); 
       
}