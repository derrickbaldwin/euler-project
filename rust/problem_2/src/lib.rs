/*

Problem 2:

Each new term in the Fibonacci sequence is generated by adding 
the previous two terms. By starting with 1 and 2, the 
first 10 terms will be:

1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...

By considering the terms in the Fibonacci sequence whose values 
do not exceed four million, find the sum of the even-valued terms.

*/

#[derive(Debug)]
pub struct Fibonacci { x: (i64, i64) }

impl Fibonacci {
    pub fn new() -> Fibonacci {
        Fibonacci { x: (0, 1)}
    }
}

impl Iterator for Fibonacci {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        self.x = (self.x.1, self.x.0 + self.x.1);
        Some(self.x.1)
    }
}

pub fn sum_even_fibs_below_number(n: i64) -> i64 {
    Fibonacci::new().take_while(|x| x < &n)
                    .filter(|x| x % 2 == 0)
                    .sum()
}


#[cfg(test)]
mod tests {
    use::sum_even_fibs_below_number;

    #[test]
    fn fib_sums_below_1() {
        assert_eq!(sum_even_fibs_below_number(1), 0);
    }

    #[test]
    fn fib_sums_below_2() {
        assert_eq!(sum_even_fibs_below_number(2), 0);
    }

    #[test]
    fn fib_sums_below_10() {
        assert_eq!(sum_even_fibs_below_number(10), 10);
    }

    #[test]
    fn fib_sums_below_90() {
        assert_eq!(sum_even_fibs_below_number(90), 44);
    }
}
