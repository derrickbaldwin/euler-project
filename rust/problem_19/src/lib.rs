/*

Problem 19:

You are given the following information, but you may 
prefer to do some research for yourself.

    1 Jan 1900 was a Monday.
    Thirty days has September,
    April, June and November.
    All the rest have thirty-one,
    Saving February alone,
    Which has twenty-eight, rain or shine.
    And on leap years, twenty-nine.
    A leap year occurs on any year evenly divisible by 4, but not 
    on a century unless it is divisible by 400.

How many Sundays fell on the first of the month during the 
twentieth century (1 Jan 1901 to 31 Dec 2000)?

*/

/*
What we know .. 
    Jan 1 1901 falls on a Tuesday.
    The 100 years is 25 cycles of (3 normal year + leap year)

    zipped day codes [1-7] over monthly day codes 
    [1-(28,29,30,31)] for the 100 yr period.

    matched (1,6) tuples: (start-of-amonth, Tues).  
    Note: Sunday was 6, since we started with a Tues     
*/

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0)  && (year % 100 != 0) || (year % 400 == 0)   
}

pub fn days_in_year(year: i32) -> i32 {
    match is_leap_year(year) {
        false => 365,
        true  => 366 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
