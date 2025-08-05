use std::process::exit;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rug::Integer;

use crate::maths::multiplicative_persistence;

mod maths;

const LIMIT: usize = 1000;

const fn generate_numbers<const LIMIT: usize>() -> [usize; LIMIT] {
    let mut numbers: [usize; LIMIT] = [0usize; LIMIT];

    let mut counter: usize = 0;
    while counter < LIMIT {
        numbers[counter] = counter + 1;
        counter += 1;
    }

    numbers
}

fn main() {
    // 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

    const NUMBERS: [usize; LIMIT] = generate_numbers();

    NUMBERS.iter().for_each(|&seven| {
        NUMBERS.par_iter().for_each(|&eight| {
            NUMBERS.iter().for_each(|&nine| {
                let result: String = format!(
                    "2{}{}{}",
                    "7".repeat(seven),
                    "8".repeat(eight),
                    "9".repeat(nine)
                );

                let number: Integer = Integer::from_str_radix(&result, 10).unwrap();
                let steps: i8 = multiplicative_persistence(number);

                if steps > 11 {
                    println!("{result} has a multiplicative persistence of {steps}");

                    exit(0);
                }
            });
        });

        println!("{seven}/{LIMIT}");
    });

    println!("No number with a multiplicative persistence greater than 11 was found.");
}
