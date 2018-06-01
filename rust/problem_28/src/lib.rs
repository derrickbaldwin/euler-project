/*

Problem 28:

Starting with the number 1 and moving to the right in a 
clockwise direction a 5 by 5 spiral is formed as follows:

21 22 23 24 25
20  7  8  9 10
19  6  1  2 11
18  5  4  3 12
17 16 15 14 13

It can be verified that the sum of the numbers on the 
diagonals is 101.

What is the sum of the numbers on the diagonals in a 1001 
by 1001 spiral formed in the same way?
*/

pub fn spiral_diagonal_sum(n: i64) -> i64 {
    let z = 4 * n * n - 6 * n + 6;
    match n {
        1 => 1,
        _ => z + spiral_diagonal_sum(n - 2)
    }
}

#[cfg(test)]
mod tests {
    use spiral_diagonal_sum;

    #[test]
    fn diagonal_sum_5() {
        assert_eq!(spiral_diagonal_sum(5), 101);
    }
}
