/*

Problem 27

Euler discovered the remarkable quadratic formula:

n2+n+41

It turns out that the formula will produce 40 primes for the 
consecutive integer values 0≤n≤39
However, when n=40,402+40+41=40(40+1)+41 is divisible by 41, 
and certainly when n=41,412+41+41

is clearly divisible by 41.

The incredible formula n2−79n+1601
was discovered, which produces 80 primes for the consecutive 
values 0≤n≤79

. The product of the coefficients, −79 and 1601, is −126479.

Considering quadratics of the form:

    n2+an+b

, where |a|<1000 and |b|≤1000

where |n|
is the modulus/absolute value of n
e.g. |11|=11 and |−4|=4

Find the product of the coefficients, a and b, for the 
quadratic expression that produces the maximum number of 
primes for consecutive values of n, starting with n=0.

*/

#[derive(Debug,Copy,Clone)]
pub struct Coefficients { 
    pub a: i64, 
    pub b: i64,
    pub c: i64,
}

impl Coefficients {
    pub fn quadratic(&mut self, n: i64) -> i64 {
        self.a * n * n + self.b * n + self.c        
    }
}

pub fn is_prime(n: i64) -> bool {
    if n < 1 {
        return false;
    }    
    let factor_range = (n as f64).sqrt() as i64;
    for i in 2..factor_range {
        if n % i == 0 {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use Coefficients;
    use is_prime;

    #[test]
    fn euler_quadratic_0() {
        let mut q = Coefficients { a: 1, b: 1, c: 41};
        assert_eq!(q.quadratic(0), 41)
    }

    #[test]
    fn euler_quadratic_0_is_prime() {
        let mut q = Coefficients { a: 1, b: 1, c: 41};
        assert_eq!(is_prime(q.quadratic(0)), true)
    }
}
