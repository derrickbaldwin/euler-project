#![feature(iterator_step_by)]
extern crate problem_59;
extern crate itertools;

use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::zip;


fn is_pwd_candidate(n: i32) -> bool {
    // english language ascii values 
    n > 31 && n < 123
}


fn main() {
    // get the cipher
    let file = BufReader::new(File::open("p059_cipher.txt").unwrap());
    let line_of_code:String = file.lines()
                           .map(|l| { l.unwrap() })
                           .collect(); 
    let codes = line_of_code.split(",").collect::<Vec<_>>();
    let cipher = codes.iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();

    // search for 1st, 2nd, and 3rd password candidates  
    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    for n in 97..123 {
        if cipher.iter().step_by(3).all(|x| is_pwd_candidate(n ^ x)) {
            a.push(n);
        }
        if cipher[1..].iter().step_by(3).all(|x| is_pwd_candidate(n ^ x)) {
            b.push(n);
        }
        
        if cipher[2..].iter().step_by(3).all(|x| is_pwd_candidate(n ^ x)) {
            c.push(n);
        }
    }
    println!("{:?}", a);  // [102, 103]
    println!("{:?}", b);  // [108, 110, 111]
    println!("{:?}", c);  // [100]

    // eligible passwords
    let _password1 = vec![102, 108, 100];
    let _password2 = vec![102, 110, 100];
    let _password3 = vec![102, 111, 100];
    let _password4 = vec![103, 108, 100];
    let _password5 = vec![103, 110, 100];
    let password6 = vec![103, 111, 100];
    
    // visually inspect the text and select best english language output
    let mut v = Vec::new();
    for (a, b) in zip(&cipher, password6.iter().cycle()) {
        v.push(a ^ b);
    }
    println!("{:?}", v.iter().map(|&x| char::from(x as u8)).collect::<Vec<_>>());
    
    // print sum of ascii values
    println!("{:?}", v.iter().sum::<i32>());
}