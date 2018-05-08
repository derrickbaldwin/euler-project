/*

Problem 12:

The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:

1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...

Let us list the factors of the first seven triangle numbers:

     1: 1
     3: 1,3
     6: 1,2,3,6
    10: 1,2,5,10
    15: 1,3,5,15
    21: 1,3,7,21
    28: 1,2,4,7,14,28

We can see that 28 is the first triangle number to have over 
five divisors.

What is the value of the first triangle number to have over 
five hundred divisors?

What we know!!!

Triangular series is n * (n+1) / 2

Number of divisors can be computed from 
prime factors: p^a * q^b .... (a + 1)*(b + 1)

*/

use std::collections::btree_map::BTreeMap;

#[derive(Debug)]
pub struct TriangularNums {
    x: i64
}

impl Iterator for TriangularNums {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.x += 1;
        Some(self.x * (self.x + 1) / 2 ) 
    }
}

impl TriangularNums {
    pub fn new() -> TriangularNums {
        TriangularNums { x: 0 }
    }
}

pub fn compute_triangular_limit(limit: i64) -> i64 {
    TriangularNums::new().skip_while(|x| num_of_divisors(*x) < limit).next().unwrap()
}

fn num_of_divisors(n: i64) -> i64 {
    let mut v = Vec::new();
    let mut f = 1;
    let mut z = n;
    while z > 1 {
        f += 1;
        while z % f == 0 {
            v.push(f);
            z = z / f;        
        }
    }
    number_of_factors(&v)
}

fn number_of_factors(v: &Vec<i64>) -> i64 {
    let mut count = BTreeMap::new();
    for n in v {
        *count.entry(n).or_insert(0) += 1;
    }
    
    let mut count_factors = 1;
    for (_, count) in &count {
        count_factors *= (count + 1) as i64;
    } 
    (count_factors - 1) as i64 //exclude factor 1
}


#[cfg(test)]
mod tests {
    use compute_triangular_limit;

    #[test]
    fn triangular_limit_500() {
        assert_eq!(compute_triangular_limit(5), 28);
    }
}