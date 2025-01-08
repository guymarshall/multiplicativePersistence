use crate::maths::{contains_only_single_digit_factors, multiplicative_persistence};
use rayon::prelude::*;
use std::process::exit;

mod maths;

// 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

const START: i64 = 278787230000000;
const CHUNK_SIZE: i64 = 200_000_000;
const STEP: usize = CHUNK_SIZE as usize;

fn main() {
    #[allow(clippy::infinite_iter)]
    (START..).step_by(STEP).for_each(|first: i64| {
        let numbers: Vec<i64> = (first..first + CHUNK_SIZE).collect();

        numbers.par_iter().for_each(|number: &i64| {
            let multiplicative_persistence: i32 = multiplicative_persistence(*number);

            if multiplicative_persistence > 10 {
                println!(
                    "{number} has a multiplicative persistence of at least 11! Checking factors..."
                );

                let factors_are_all_single_digits: bool =
                    contains_only_single_digit_factors(*number);

                if factors_are_all_single_digits {
                    println!("{number}");
                    exit(0);
                }
            }
        });

        println!("{first}");
    });
}
