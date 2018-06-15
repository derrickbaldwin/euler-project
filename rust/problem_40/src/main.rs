
extern crate problem_40;


fn main() {
    // pre-allocate bytes to string
    let mut champernowne_constant = String::with_capacity(1_200_000);
    
    // create at least 1_000_000 char string 
    for n in 0..200_000 {
        champernowne_constant.push_str(&n.to_string());
    }
    
    // convert to vec of digits
    let d = champernowne_constant.chars()
                                 .collect::<Vec<_>>()
                                 .into_iter()
                                 .map(|c| c.to_digit(10).unwrap())
                                 .collect::<Vec<_>>();
    // accumulate product of base 10 power index of digits
    println!("{}", (0..6).fold(1, |acc, x| acc * &d[10usize.pow(x)]));
}