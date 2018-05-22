extern crate problem_19;
extern crate itertools;

use itertools::concat;
use problem_19::days_in_year;


fn main() {
    let days_in_week = vec![1, 2, 3, 4, 5, 6, 7];
    let year = concat(vec![ 
        (1..32).collect::<Vec<i32>>(),
        (1..29).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..31).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..31).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..31).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..31).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
    ]);
    let leap_year = concat(vec![ 
        (1..32).collect::<Vec<i32>>(),
        (1..30).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..31).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..31).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..31).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
        (1..31).collect::<Vec<i32>>(),
        (1..32).collect::<Vec<i32>>(),
    ]);
    
    let four_year_cycle = concat(vec![ year.clone(), 
                                       year.clone(), 
                                       year.clone(), 
                                       leap_year.clone()]
                                );
    
    let mut v = Vec::new();
    for year in 1901..2001 {
        v.push(days_in_year(year));
    }
    let n: i32 = v.iter().sum();
    
    let monthly_day: Vec<i32> = four_year_cycle.into_iter().cycle().take(n as usize).collect();
    let weekday: Vec<i32> = days_in_week.into_iter().cycle().take(n as usize).collect();
    let period_tuples: Vec<(&i32, &i32)> = monthly_day.iter().zip(weekday.iter()).collect();

    let mut counter = 0;
    for x in &period_tuples {
        if x.0 == &1 && x.1 == &6 {
            counter += 1;
        }
    }
    println!("{}", counter);
}
