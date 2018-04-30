/*

Problem 5:

2520 is the smallest number that can be divided by 
each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly 
divisible by all of the numbers from 1 to 20?

*/

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let t = a;
        a = b;
        b = t % b;
    }
    a
}

pub fn smallest_multiple(n: u64) -> u64 {
    (2..n+1).fold(1, |acc, i| { acc * i / gcd(i, acc )})
}


#[cfg(test)]
mod tests {
    use smallest_multiple;

    #[test]
    fn smallest_multiple_10() {
        assert_eq!(smallest_multiple(10), 2520);
    }
}
