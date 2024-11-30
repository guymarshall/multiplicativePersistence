#![forbid(unsafe_code)]

use std::process::exit;
use std::str::FromStr;
use std::time::Instant;
use num::BigInt;
use crate::maths::{contains_only_single_digit_factors, get_prime_factors, multiplicative_persistence};

mod maths;

// 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

// 277777788888899

fn main() {
    let start_time: Instant = Instant::now();

    let mut count: i32 = 1000;
    const LAST_NUMBER: &str = "277779889788879";
    let mut number: BigInt = BigInt::from_str(LAST_NUMBER).unwrap();

    while count > 0 {
        let multiplicative_persistence: i32 = multiplicative_persistence(number.clone());

        if multiplicative_persistence == 11 {
            println!("Numbers left: {count}");
            count -= 1;

            let prime_factors: Vec<BigInt> = get_prime_factors(number.clone());
            let contains_only_single_digit_factors: bool = contains_only_single_digit_factors(prime_factors);
            if contains_only_single_digit_factors {
                println!("Number found!");
                println!("{number}");
                exit(0);
            }
        }

        number += 1;
    }

    println!("{}", number);

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
