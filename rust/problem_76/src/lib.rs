/*
Problem 76:

It is possible to write five as a sum in exactly six different ways:

4 + 1
3 + 2
3 + 1 + 1
2 + 2 + 1
2 + 1 + 1 + 1
1 + 1 + 1 + 1 + 1

How many different ways can one hundred be written as a sum of at 
least two positive integers?

*/


pub fn counting_summations(parts: &[usize], sum: usize) -> usize {
    let size = sum + 1;
    let mut combo = vec![0; size];
    combo[0] = 1;
    for &part in parts {
        for amount in part..size {
            combo[amount] += combo[amount - part];
        }
    }
    combo[sum]
}

#[cfg(test)]
mod tests {
    use counting_summations;

    #[test]
    fn counting_summations_5() {
        assert_eq!(counting_summations(&[1,2,3,4], 5), 6);
    }
}
