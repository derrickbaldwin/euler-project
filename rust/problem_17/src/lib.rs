/*

Problem 17:

If the numbers 1 to 5 are written out in words: one, two, 
three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 
letters used in total.

If all the numbers from 1 to 1000 (one thousand) inclusive 
were written out in words, how many letters would be used?

NOTE: Do not count spaces or hyphens. For example, 342 
(three hundred and forty-two) contains 23 letters and 
115 (one hundred and fifteen) contains 20 letters. 

The use of "and" when writing out numbers is in compliance 
with British usage.

*/

pub fn word_nums(n: u32) -> String {
    const DIGITS: [&str; 20] = [
        "zero", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine", "ten",
        "eleven", "twelve", "thirteen", "fourteen", "fifteen",
        "sixteen", "seventeen", "eighteen", "nineteen"
    ];

    const TENS: [&str; 10] = [
        "", "", "twenty", "thirty", "forty",
        "fifty", "sixty", "seventy", "eighty", "ninety"
    ];

    match n {
        0...19     => String::from(DIGITS[n as usize]),
        20...99    => match n % 10 {
                        0 => String::from(TENS[(n / 10) as usize]),
                        _ => format!("{}-{}", word_nums(n - n % 10), word_nums(n % 10))
                      },
        100...999  => match n % 100 {
                        0 => format!("{} hundred", String::from(DIGITS[(n / 100) as usize])),
                        _ => format!("{} hundred and {}", word_nums(n / 100), word_nums(n % 100))
                      }, 
        1000       => String::from("one thousand"),            
        _          => String::from("Valid Range: [0, 1000]")
    }
}

pub fn count_letters(num: String) -> usize {
   num.chars().filter(|c| c.is_alphabetic()).count()
}

pub fn word_nums_letter_count(n: u32) -> usize {
    count_letters(word_nums(n))
}

pub fn total_letter_count(n: u32) -> usize {
    (1..n+1).map(|x| word_nums_letter_count(x))
            .collect::<Vec<usize>>()
            .iter()
            .sum()
}


#[cfg(test)]
mod tests {
    use word_nums;
    use word_nums_letter_count;

    #[test]
    fn word_nums_115() {
        assert_eq!(word_nums(115), String::from("one hundred and fifteen") );
    }

    #[test]
    fn word_nums_342() {
        assert_eq!(word_nums(342), String::from("three hundred and forty-two") );
    }

    #[test]
    fn word_nums_letter_count_115() {
        assert_eq!(word_nums_letter_count(115), 20);
    }

    #[test]
    fn word_ums_letter_count_342() {
        assert_eq!(word_nums_letter_count(342), 23);
    }


}
