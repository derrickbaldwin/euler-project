extern crate problem_33;

use problem_33::gcd;

fn main() {
    let mut ys = Vec::new();
    let mut zs = Vec::new();
    let base = 10;
    for x in 1..base {
        for y in 1..x {
            for z in 1..y {
                if (base * x * z  + y * z) == (base * z * y + x * y) {
                    ys.push(y);
                    zs.push(z);
                } 
            }
        }
    }
    let product_ys = ys.iter().fold(1, |acc, y| acc * y);
    let product_zs = zs.iter().fold(1, |acc, z| acc * z);
    println!("{}", product_ys / gcd( product_zs, product_ys));

}

