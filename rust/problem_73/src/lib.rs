/*
Problem 73:

Consider the fraction, n/d, where n and d are positive integers. 
If n<d and HCF(n,d)=1, it is called a reduced proper fraction.

If we list the set of reduced proper fractions for d ≤ 8 in 
ascending order of size, we get:

1/8, 1/7, 1/6, 1/5, 1/4, 2/7, 1/3, 3/8, 2/5, 3/7, 1/2, 4/7, 3/5, 
5/8, 2/3, 5/7, 3/4, 4/5, 5/6, 6/7, 7/8

It can be seen that there are 3 fractions between 1/3 and 1/2.

How many fractions lie between 1/3 and 1/2 in the sorted set 
of reduced proper fractions for d ≤ 12,000?
*/

pub fn fractional_range_count(x: (u64,u64), y: (u64, u64), limit: u64) -> u64 {
    let num = x.0 + y.0;
    let den = x.1 + y.1;
    let median = (num, den);
    match den {
        den if den > limit => 0,
        _              => 1 + fractional_range_count(x, median, limit) 
                            + fractional_range_count(median, y, limit),
    }
} 

#[cfg(test)]
mod tests {
    use fractional_range_count;

    #[test]
    fn range_count_test() {
        assert_eq!(fractional_range_count((1,3), (1,2), 8), 3);
    }
}
