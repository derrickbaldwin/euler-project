/*

Problem 14: 

The following iterative sequence is defined for the set
of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate 
the following sequence:
13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1

It can be seen that this sequence (starting at 13 and 
finishing at 1) contains 10 terms. Although it has not 
been proved yet (Collatz Problem), it is thought that 
all starting numbers finish at 1.

Which starting number, under one million, produces the 
longest chain?

NOTE: Once the chain starts the terms are allowed to 
go above one million.

*/

pub fn collatz(n: u64) -> Option<u64> {
    let mut count = 1;
    let mut c = n; 
    if n == 0 {
        return None;
    }
    while c != 1 {
        c = collatz_state(c);
        count += 1;
    }
    Some(count)
}

fn collatz_state(n: u64) -> u64 {
    match n % 2 == 0 {
        true  => n / 2, 
        false => 3 * n + 1
    }
}


pub fn max_collatz(limit: u64) -> u64 {
    let collatz_seq = (1..limit).map(|n| collatz(n).unwrap())
                                .collect::<Vec<_>>();
    let max_seq = collatz_seq.iter()
                             .max()
                             .unwrap();
    let idx_max_seq = collatz_seq.iter()
                                 .rposition(|&x| x == *max_seq)
                                 .unwrap();

    idx_max_seq as u64  + 1   
}

#[cfg(test)]
mod tests {
    use collatz;

    #[test]
    fn collatz_13() {
        assert_eq!(collatz(13), Some(10));
    }
}
