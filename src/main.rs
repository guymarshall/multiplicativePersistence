#![forbid(unsafe_code)]

use std::str::FromStr;
use std::time::Instant;
use num::BigInt;
use crate::maths::{contains_only_single_digit_factors, get_prime_factors, multiplicative_persistence};

mod maths;

fn main() {
    let start_time: Instant = Instant::now();

    let mut count: i32 = 1000;
    let mut number: BigInt = BigInt::from_str("277777788888899").unwrap();

    while count > 0 {
        let multiplicative_persistence: i32 = multiplicative_persistence(number.clone());

        if multiplicative_persistence == 11 {
            println!("Numbers left: {count}");
            count -= 1;

            let prime_factors: Vec<BigInt> = get_prime_factors(number.clone());
            let contains_only_single_digit_factors: bool = contains_only_single_digit_factors(prime_factors);
            if contains_only_single_digit_factors {
                println!("{number}");
            }
        }

        number += 1;
    }

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
