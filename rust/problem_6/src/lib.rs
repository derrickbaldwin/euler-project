/*

Problem 6:

The sum of the squares of the first ten natural 
numbers is,

12 + 22 + ... + 102 = 385

The square of the sum of the first ten natural 
numbers is,

(1 + 2 + ... + 10)2 = 552 = 3025

Hence the difference between the sum of the squares 
of the first ten natural numbers and the square of 
the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares 
of the first one hundred natural numbers and the 
square of the sum.

*/

pub fn algebraic_sum_of_squares(n: u64) -> u64 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn iterative_sum_of_squares(n: u64) -> u64 {
    (1..n + 1).map(|i| i * i).fold(0, |acc, i| acc + i)
}

pub fn algebraic_square_of_sums(n: u64) -> u64 {
     n * n * (n + 1) * (n + 1) / 4
}

pub fn iterative_square_of_sums(n: u64) -> u64 {
    (1..n + 1).fold(0, |acc, i| acc + i).pow(2)
}

pub fn iterative_sum_square_difference(n: u64) -> u64 {
    iterative_square_of_sums(n) - iterative_sum_of_squares(n) 
}

pub fn algebraic_sum_square_difference(n: u64) -> u64 {
    algebraic_square_of_sums(n) - algebraic_sum_of_squares(n)
}

#[cfg(test)]
mod tests {

    use algebraic_square_of_sums;
    use algebraic_sum_of_squares;
    use algebraic_sum_square_difference;
    use iterative_square_of_sums;
    use iterative_sum_of_squares;
    use iterative_sum_square_difference;

    #[test]
    fn algebraic_square_of_sums_10() {
        assert_eq!(algebraic_square_of_sums(10), 3025);
    }

    #[test]
    fn iterative_square_of_sums_10() {
        assert_eq!(iterative_square_of_sums(10), 3025);
    }

    #[test]
    fn algebraic_sum_of_squares_10() {
        assert_eq!(algebraic_sum_of_squares(10), 385);
    }

    #[test]
    fn iterative_sum_of_squares_10() {
        assert_eq!(iterative_sum_of_squares(10), 385);
    }

    #[test]
    fn algebraic_sum_square_difference_10() {
        assert_eq!(algebraic_sum_square_difference(10), 2640);
    }

    #[test]
    fn iterative_sum_square_difference_10() {
        assert_eq!(iterative_sum_square_difference(10), 2640);
    }

}
