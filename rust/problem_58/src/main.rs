extern crate problem_58;

use problem_58::north_east;
use problem_58::north_west;
use problem_58::south_west;

fn main() {
    let mut corner_primes = 0;
    let mut average: f32 = 1.0;
    let mut d = 1;
    let mut n = 2;
    while average > 0.10 {
        corner_primes += north_east(d, n) + north_west(d, n) + south_west(d, n);
        d += n * 4;
        n += 2;
        average = corner_primes as f32 / (2f32 * n as f32); 
    }
    println!("{}", n + 1);
}
