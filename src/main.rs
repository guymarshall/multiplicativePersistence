use crate::maths::{
    contains_only_single_digit_factors, get_prime_factors, multiplicative_persistence,
};
use std::process::exit;

mod maths;

// 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

fn main() {
    let mut number: i64 = 277_778_540_000_000;

    loop {
        let multiplicative_persistence: i32 = multiplicative_persistence(number);

        if number % 10_000_000 == 0 {
            println!("{number}");
        }

        if multiplicative_persistence == 11 {
            println!("{number} has a multiplicative persistence of 11! Checking factors...");

            let factors: Vec<i64> = get_prime_factors(number);

            let factors_are_all_single_digits: bool = contains_only_single_digit_factors(factors);

            if factors_are_all_single_digits {
                println!("{number}");
                exit(0);
            }
        }

        number += 1;
    }
}
