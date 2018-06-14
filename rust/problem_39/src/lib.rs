/*

Problem 39:

If p is the perimeter of a right angle triangle with 
integral length sides, {a,b,c}, there are exactly 
three solutions for p = 120.

{20,48,52}, {24,45,51}, {30,40,50}

For which value of p ≤ 1000, is the number of solutions maximised?

*/

use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[derive(Debug, Clone)]
pub struct RightTriangle {
    solutions: i32,
    perimeter: i32
}


pub fn brute_force_max_right_triangles(limit: i32) -> Option<RightTriangle> {
    let mut max_p = 0;
    let mut max_solutions = 0;
    let mut heap = BinaryHeap::new(); 

    for p in 1i32..limit+1 {
        let mut counter = 0;
        for a in 1i32..p {
            for b in 1i32..a {
                let mut c = p - a - b;
                if a * a + b * b == c * c {
                    counter += 1;
                }
            }
        }
        heap.push(RightTriangle { solutions: counter, perimeter: p }); 
        
    }
    let mut int_right_triangles = heap.clone();
    int_right_triangles.pop()
}


#[cfg(test)]
mod tests {
    use RightTriangle;
    use brute_force_max_right_triangles;

    #[test]
    fn integer_triangles_limit_120() {
        assert_eq!(brute_force_max_right_triangles(120), 
                     Some(RightTriangle { solutions: 3, perimeter: 120 }));
    }
}
