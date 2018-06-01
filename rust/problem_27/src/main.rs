extern crate problem_27;

use problem_27::Coefficients;
use problem_27::is_prime;

fn main() {
    let mut max_running_primes = 40;      
    let mut max_running_primes_coefficients = 
                    Coefficients {a: 1, b: 1, c: 41 };
    
    let primes = (1..1000).filter(|&b| is_prime(b))
                          .collect::<Vec<_>>(); 
    let a: i64 = 1;
    for b in -1000i64..0 {
        for c in &primes {
            let mut n = 0;
            let mut q = Coefficients {a: a, b: b, c: *c }; 
            while is_prime(q.quadratic(n)) {
                n += 1;
            }
            if n > max_running_primes {
                max_running_primes = n;
                max_running_primes_coefficients = 
                    Coefficients { a: a, b: b, c: *c};
            }
        }
    }    
    println!("length {}, {:?}, b * c = {}", 
                max_running_primes, 
                max_running_primes_coefficients, 
                max_running_primes_coefficients.b * max_running_primes_coefficients.c);
}
