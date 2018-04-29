
/*

Problem 1:

If we list all the natural numbers below 10 that are 
multiples of 3 or 5, we get 3, 5, 6 and 9. 

The sum of these multiples is 23.

Find the sum of all the multiples of 3 or 5 below 1000.
*/

pub fn multiples_of_3_and_5(n: i64) -> i64 {
    (3..n).filter(|x| x % 3 == 0 || x % 5 == 0)
          .collect::<Vec<i64>>()
          .iter()
          .sum()
}

pub fn multiples_of_3_and_5_v2(n: i64) -> i64 {
    let mut total: i64 = 0;
    let mut i: i64 = 3;
    while i < n {
        if i % 3 == 0 || i % 5 == 0 {
            total += i;
        }
        i += 1;
    }
    total
}


#[cfg(test)]
mod tests {
    use multiples_of_3_and_5;
    
    #[test]
    fn it_works() {
        assert_eq!(multiples_of_3_and_5(10), 23);
    }
}
