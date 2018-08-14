/*
Problem 62:

The cube, 41063625 (3453), can be permuted to produce two other 
cubes: 56623104 (3843) and 66430125 (4053). In fact, 41063625 is 
the smallest cube which has exactly three permutations of its 
digits which are also cube.

Find the smallest cube for which exactly five permutations of 
its digits are cube.

127 035 954 683

*/

use std::iter::Iterator;
use std::iter::FromIterator;

pub fn cube(n: u64) -> u64 {
    n * n * n
}

pub fn sorted_cube(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    String::from_iter(chars)
}


#[cfg(test)]
mod tests {
    use cube;

    #[test]
    fn cube_345() {
        assert_eq!(cube(345), 41063625);
    }

    #[test]
    fn cube_384() {
        assert_eq!(cube(384), 56623104);
    }

    #[test]
    fn cube_405() {
        assert_eq!(cube(405), 66430125);
    }
}
