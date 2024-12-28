use crate::maths::{contains_only_single_digit_factors, multiplicative_persistence};
use std::process::exit;

mod maths;

// 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

const UPDATE_FREQUENCY: i64 = 10_000_000;

fn main() {
    #[allow(clippy::infinite_iter)]
    (277781550000000..).for_each(|number: i64| {
        let multiplicative_persistence: i32 = multiplicative_persistence(number);

        if number % UPDATE_FREQUENCY == 0 {
            println!("{number}");
        }

        if multiplicative_persistence > 10 {
            println!(
                "{number} has a multiplicative persistence of at least 11! Checking factors..."
            );

            let factors_are_all_single_digits: bool = contains_only_single_digit_factors(number);

            if factors_are_all_single_digits {
                println!("{number}");
                exit(0);
            }
        }
    });
}
