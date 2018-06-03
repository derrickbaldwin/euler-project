/*

Problem 31:

In England the currency is made up of pound, £, and 
pence, p, and there are eight coins in general 
circulation:

    1p, 2p, 5p, 10p, 20p, 50p, £1 (100p) and £2 (200p).

It is possible to make £2 in the following way:

    1×£1 + 1×50p + 2×20p + 1×5p + 1×2p + 3×1p

How many different ways can £2 be made using any number 
of coins?

*/

pub fn count_change(coins: &[usize], sum: usize) -> usize {
    let size = sum + 1;
    let mut combo = vec![0; size];
    combo[0] = 1;
    for &coin in coins {
        for amount in coin..size {
            combo[amount] += combo[amount - coin];
        }
    }
    combo[sum]
}

#[cfg(test)]
mod tests {
    use count_change;

    #[test]
    fn count_change_100() {

        assert_eq!(count_change(&[1, 5, 10, 25, 50], 100), 292);
    }
}
