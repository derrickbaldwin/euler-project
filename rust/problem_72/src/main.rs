#![feature(iterator_step_by)]
extern crate problem_72;

fn main() {
    let limit = 1_000_000;
    let mut phi: Vec<usize> = (0..limit+1).collect();
    for i in 2..limit+1 {
        if phi[i] == i {
            for j in (i..limit+1).step_by(i as usize) {
                phi[j] = phi[j] / i * (i - 1);
            }  
        }
    }
    println!("{}", phi.iter().sum::<usize>());   
}
