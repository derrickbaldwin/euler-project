/*

Problem 22:

Using names.txt (right click and 'Save Link/Target As...'), 
a 46K text file containing over five-thousand first names, 
begin by sorting it into alphabetical order. Then working 
out the alphabetical value for each name, multiply this 
value by its alphabetical position in the list to obtain 
a name score.

For example, when the list is sorted into alphabetical 
order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is 
the 938th name in the list. So, COLIN would obtain a score 
of 938 × 53 = 49714.

What is the total of all the name scores in the file?

*/

use std::fs::File;
use std::io::Read;

pub fn score(name: &str) -> u32 {
    name.chars()
         .map(|c| c as u32 - 65 + 1)
         .sum()
}

pub fn name_scores(file_name: &str) -> u32 {
    let mut f = File::open(file_name).expect("File not found");
    let mut txt = String::new();
    f.read_to_string(&mut txt).expect("Could not read file");
        
    let mut names: Vec<&str> = txt.split(",")
                                  .map(|s| s.trim().trim_matches('\"'))
                                  .collect();
    names.sort();
    names.iter().enumerate()
                .map(|(rank, name)| score(name) * (rank as u32 + 1))
                .sum()
} 


#[cfg(test)]
mod tests {
    use score;
    
    #[test]
    fn name_score() {
        assert_eq!(score("COLIN"), 53);
    }
}
