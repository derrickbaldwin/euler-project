extern crate problem_48;

fn main() {
    let mut result:u64 = 0;
    let m: u64 = 10_000_000_000;
    for i in 1..1000 {
        let mut tmp = i;
        for j in 1..i {
            tmp *= i;
            tmp %= m; 
        }
        result += tmp;
        result %= m; 
    }
    println!("{}", result);
}