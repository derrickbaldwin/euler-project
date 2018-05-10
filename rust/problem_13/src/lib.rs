
pub fn first_10_digits(nums: String) -> String {
    nums.lines()
        .map(|d| &d[0..15])
        .filter_map(|s| s.parse::<u64>().ok())
        .sum::<u64>()
        .to_string()[0..10]
        .to_string()
}




