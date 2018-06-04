/*

Problem 32:


We shall say that an n-digit number is pandigital if it
makes use of all the digits 1 to n exactly once; for 
example, the 5-digit number, 15234, is 1 through 5 pandigital.

The product 7254 is unusual, as the identity, 39 × 186 = 7254, 
containing multiplicand, multiplier, and product is 1 
through 9 pandigital.

Find the sum of all products whose multiplicand/multiplier/product 
identity can be written as a 1 through 9 pandigital.

HINT: Some products can be obtained in more than one way so be 
sure to only include it once in your sum.

*/

use std::collections::HashSet;

pub fn is_product_pandigital(num: &str) -> bool {
    let mut unique = HashSet::new();
    num.chars().all(|d| d != '0' && unique.insert(d) )         
}

#[cfg(test)]
mod tests {
    use is_product_pandigital;

    #[test]
    fn pandigital_products_39_186() {
        assert_eq!(is_product_pandigital("391867254"), true);
    }
}
