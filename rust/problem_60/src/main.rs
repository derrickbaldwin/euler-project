extern crate problem_60;
extern crate primal;

use problem_60::prime_pairs;

const LIMIT: usize = 10_000;

fn main() {
    let primes = primal::Primes::all().take_while(|p| *p < LIMIT)
                                      .collect::<Vec<_>>();
    'outer: for a in &primes {
        for b in &primes {
            if a >= b {
                continue
            }
            if prime_pairs(*a, *b) {
                for c in &primes {
                    if b >= c {
                        continue
                    }
                    if prime_pairs(*a, *c) && prime_pairs(*b, *c) {
                        for d in &primes {
                            if c >= d {
                                continue
                            }
                            if prime_pairs(*a, *d) && prime_pairs(*b, *d) && prime_pairs(*c, *d) {
                                for e in &primes {
                                    if d >= e {
                                        continue
                                    }
                                    if prime_pairs(*a, *e) && prime_pairs(*b, *e) && prime_pairs(*c, *e) 
                                        && prime_pairs(*d, *e) {
                                        println!("{} {} {} {} {} -> {}", a, b, c, d, e, a + b + c + d + e);
                                        break 'outer;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
