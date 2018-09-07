extern crate problem_71;

fn main() {
    let limit = 1_000_000;
    let mut left_num = 2;
    let mut left_den = 5;
    let boundary_num = 3;
    let boundary_den = 7;
    let mut counter = 0;
    while left_den + boundary_den <= limit {
        left_num += boundary_num;
        left_den += boundary_den;
        counter += 1;
    }
    println!("{} / {} -> {}", left_num, left_den, counter);
}

