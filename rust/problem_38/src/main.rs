extern crate problem_38;

use problem_38::is_pandigital;

pub fn main() {
    let mut candidates = Vec::new();
    for x in 1000..10000 {
        let mut concat_nums = Vec::new();
        concat_nums.push((x * 1).to_string());
        concat_nums.push((x * 2).to_string());
        let mut t = &concat_nums.join("");
        candidates.push(t.clone());
    }
    let pandigital_multiples = candidates.iter()
                                         .filter(|x| is_pandigital(x))
                                         .collect::<Vec<_>>();
    println!("{:?}", &pandigital_multiples.last().unwrap());
}


