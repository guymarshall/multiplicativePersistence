use crate::maths::{contains_only_single_digit_factors, multiplicative_persistence};
use rayon::prelude::*;
use std::process::exit;
use crate::memory::calculate_chunk_size;

mod maths;
mod memory;

// 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899
// TODO: once each chunk is processed, write back to the file to change
// TODO: this to the latest number checked
const START: i64 = 280826826401624;

fn main() {
    let chunk_size: i64 = calculate_chunk_size() as i64;
    let step: usize = chunk_size as usize;

    #[allow(clippy::infinite_iter)]
    (START..).step_by(step).for_each(|first: i64| {
        let numbers: Vec<i64> = (first..first + chunk_size).collect();

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
