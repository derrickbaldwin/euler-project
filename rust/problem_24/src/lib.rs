/*

Problem 24:

A permutation is an ordered arrangement of objects. For example, 
3124 is one possible permutation of the digits 1, 2, 3 and 4. 
If all of the permutations are listed numerically or alphabetically, 
we call it lexicographic order. 

The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the 
digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

-----------------------------------------------------

L Algorithm Description:

    Find largest index i such that array[i − 1] < array[i].
    (If no such i exists, then this is already the last permutation.)

    Find largest index j such that j ≥ i and array[j] > array[i − 1].

    Swap array[j] and array[i − 1].

    Reverse the suffix starting at array[i].

*/


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


