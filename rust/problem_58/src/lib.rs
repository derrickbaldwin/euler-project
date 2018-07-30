

extern crate primal;

use primal::is_prime;

pub fn p(n: u64) -> i32 {
    is_prime(n) as i32
}

pub fn north_east(x: u64, y: u64) -> i32 {
    p(x + y)
}

pub fn north_west(x: u64, y: u64) -> i32 {
    p(x + 2 * y)
}

pub fn south_west(x: u64, y: u64) -> i32 {
    p(x + 3 * y)
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
