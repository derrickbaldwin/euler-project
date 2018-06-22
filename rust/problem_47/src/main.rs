extern crate problem_47;
extern crate itertools;

use problem_47::all_fours;
use problem_47::unique_prime_factors_length;
use itertools::Itertools;

fn main() {
    let limit = 150_000;
    let prime_lens = (1..limit).map(|x| unique_prime_factors_length(x))
                               .collect::<Vec<_>>();
    let mut v = Vec::new();
    for (a,b,c,d) in prime_lens.into_iter().tuple_windows() {
        v.push((a,b,c,d));
    }
    for (i,w) in v.iter().enumerate() {
        if all_fours(*w) {
            println!("found {}", i + 1);
            break;
        }
    }
}