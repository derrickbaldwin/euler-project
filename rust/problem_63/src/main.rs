#![feature(i128_type)]

fn main() {
    let mut counter = 0; 
    for i in 1..10 {
        for j in 1..22 {
            if u128::pow(i,j).to_string().len() == j as usize {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}