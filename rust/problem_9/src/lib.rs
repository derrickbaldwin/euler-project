
/*

Problem 9: 

A Pythagorean triplet is a set of three natural numbers, 
a < b < c, for which, a2 + b2 = c2

For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which 
a + b + c = 1000.

Find the product abc.

*/

pub fn find_triplet() -> Option<u32> {
    for a in 1..1_000 {
        let mut c = (a * a + 500_000 - 1_000 * a) / (1_000 - a);
        let mut b = 1000 - a - c;

        if b > a && c > b && a * a + b * b == c * c {
            return Some(a * b * c);
        }
    }
    None
}



