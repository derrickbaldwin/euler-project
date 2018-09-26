extern crate problem_78;

use problem_78::generalized_pentagonal_numbers;

fn main() {
    const MILLION: i32 = 1_000_000;    
    
    let exponents = generalized_pentagonal_numbers(250);
    let mut partitions = vec!(1);
    let state_pattern = [1, 1, -1, -1];
    let mut n: usize = 0; 
    
    loop {
        n += 1;
        let mut p = 0;
        let mut i  = 0;
        while exponents[i] <= n {
            p += partitions[n - exponents[i]] * state_pattern[(i % 4) as usize];
            i += 1;
        }
        partitions.push(p % MILLION);
        if partitions[n] == 0 {
            break;
        }
    }
    println!("{:?}", n);   
}