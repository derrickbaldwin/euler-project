extern crate itertools;

// function from euler problem 24
pub fn permutate(digits: &mut [i32; 10]) -> bool {
	let mut i: usize = digits.len() - 1;
	while i > 0 && digits[i - 1] >= digits[i] {
		i -= 1;
	}
	if i == 0 {
		return false;
	}
	let mut j: usize = digits.len() - 1;
	while digits[j] <= digits[i - 1] {
		j -= 1;
	}
	digits.swap(i - 1, j);
	digits[i .. ].reverse();
	true
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        assert_eq!(2+2, 4);
    }
}
