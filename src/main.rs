#![forbid(unsafe_code)]

use std::time::Instant;
use num::BigInt;
use crate::maths::{contains_only_single_digit_factors, get_prime_factors, multiplicative_persistence};

mod maths;

fn main() {
    let start_time: Instant = Instant::now();

    let mut count: i32 = 10;
    let mut number: BigInt = BigInt::from(277777788888899_i64);
    let mut numbers_with_persistence_eleven: Vec<BigInt> = Vec::with_capacity(count as usize);

    while count > 0 {
        let multiplicative_persistence: i32 = multiplicative_persistence(number.clone());

        if multiplicative_persistence == 11 {
            println!("Numbers left: {count}");
            numbers_with_persistence_eleven.push(number.clone());
            count -= 1;

            if contains_only_single_digit_factors(get_prime_factors(number.clone())) {
                println!("{number}");
            }
        }

        number += 1;
    }

    println!("{:?}", numbers_with_persistence_eleven);

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
