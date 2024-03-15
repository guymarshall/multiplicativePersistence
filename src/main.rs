#![forbid(unsafe_code)]

use crate::maths::{contains_zero, multiplicative_persistence};

mod maths;

fn main() {
    // get prime factors that are not larger than 7 from LARGEST
    // const LARGEST: i64 = 277777788888899;
    // println!("Hello, world!");
    
    // OLD CODE BELOW

    let mut highest_steps_count: i64 = 0;
    let mut highest_steps_number: i64 = 0;

    let start: i64 = 0;
    let finish: i64 = 277777788888899;

    (start..=finish).filter(|number| !contains_zero(*number)).for_each(|number| {
        let result: i64 = multiplicative_persistence(number);
        if result > highest_steps_count {
            highest_steps_count = result;
            highest_steps_number = number;
            println!("Upto {} so far with {} steps", number, result);
        }
    });

    println!("Highest step count: {} at {}", highest_steps_number, highest_steps_count);
}