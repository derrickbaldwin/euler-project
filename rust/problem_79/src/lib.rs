
/*
Problem 79:

A common security method used for online banking is to ask the 
user for three random characters from a passcode. For example, 
if the passcode was 531278, they may ask for the 2nd, 3rd, and 
5th characters; the expected reply would be: 317.

The text file, keylog.txt, contains fifty successful login attempts.

Given that the three characters are always asked for in order, 
analyse the file so as to determine the shortest possible 
secret passcode of unknown length.
*/

pub fn position_swap(x: u32, y: u32, source: &mut Vec<u32>) {
    let i = idx(x, source);
    let j = idx(y, source); 
    if i > j {
        source.swap(i, j);
    }
}

fn idx(n: u32, v: &mut Vec<u32>) -> usize {
    v.iter().position(|&i| i == n).unwrap()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
