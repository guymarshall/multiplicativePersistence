use crate::maths::{contains_only_single_digit_factors, multiplicative_persistence};
use std::process::exit;

mod maths;

// 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

const UPDATE_FREQUENCY: i64 = 10_000_000;

fn main() {
    let mut number: i64 = 277777788888899;

    loop {
        let multiplicative_persistence: i32 = multiplicative_persistence(number);

        if number % UPDATE_FREQUENCY == 0 {
            println!("{number}");
        }

        if multiplicative_persistence == 11 {
            println!("{number} has a multiplicative persistence of 11! Checking factors...");

            let factors_are_all_single_digits: bool = contains_only_single_digit_factors(number);

            if factors_are_all_single_digits {
                println!("{number}");
                exit(0);
            }
        }

        number += 1;
    }
}
