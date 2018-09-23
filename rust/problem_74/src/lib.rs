/*


The number 145 is well known for the property that the sum of 
the factorial of its digits is equal to 145:

1! + 4! + 5! = 1 + 24 + 120 = 145

Perhaps less well known is 169, in that it produces the longest 
chain of numbers that link back to 169; it turns out that there 
are only three such loops that exist:

169 → 363601 → 1454 → 169
871 → 45361 → 871
872 → 45362 → 872

It is not difficult to prove that EVERY starting number 
will eventually get stuck in a loop. For example,

69 → 363600 → 1454 → 169 → 363601 (→ 1454)
78 → 45360 → 871 → 45361 (→ 871)
540 → 145 (→ 145)

Starting with 69 produces a chain of five non-repeating terms, 
but the longest non-repeating chain with a starting number 
below one million is sixty terms.

How many chains, with a starting number below one million, 
contain exactly sixty non-repeating terms?
*/
use std::collections::HashMap;


static DIGIT_FACTORIAL: &'static [u64] = &[1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880];

pub fn factorial_sum(n: u64) -> u64 {
    let s = n.to_string();
    s.chars().map(|x| x.to_digit(10).unwrap())
             .collect::<Vec<_>>()
             .iter()
             .map(|&x| DIGIT_FACTORIAL[x as usize])
             .sum()
}

pub fn chain(n: u64, map: &mut HashMap<u64,u64>) -> u64 {
    if map.contains_key(&n) {
        return *map.get(&n).unwrap()
    }
    let build = chain(factorial_sum(n),  map) + 1;
    map.insert(n, build);
    build
}


#[cfg(test)]
mod tests {
    use chain;
    use std::collections::HashMap;

    #[test]
    fn chains_69_78_540() {
        let mut termination: HashMap<u64,u64> = 
        [(0,1),
        (1,1),
        (2,1),
        (145,1),
        (169,3),
        (871,2),
        (872,2),
        (1454,3),
        (40585,1),
        (45361,2),
        (45362,2),
        (363601,3)]
        .iter().cloned().collect();

        assert_eq!(chain(69, &mut termination), 5);
        assert_eq!(chain(78, &mut termination), 4);
        assert_eq!(chain(540, &mut termination), 2);
    }
}
