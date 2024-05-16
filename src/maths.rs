#![forbid(unsafe_code)]

use num::{BigInt, One, Zero};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn product(mut input: BigInt) -> BigInt {
    let mut result: BigInt = BigInt::one();

    // get each digit by mod instead of string conversion
    while input > BigInt::zero() {
        result *= &input % 10;
        input /= 10;
    }

    result
}

pub fn multiplicative_persistence(mut user_input: BigInt) -> i32 {
    let mut steps: i32 = 0;

    // 10 is the smallest double-digit number
    while user_input > BigInt::from(10) {
        user_input = product(user_input);
        steps += 1;
    }

    steps
}

pub fn get_prime_factors(mut number: BigInt) -> Vec<BigInt> {
    let mut factors: Vec<BigInt> = vec![];

    while &number % BigInt::from(2) == BigInt::zero() {
        factors.push(BigInt::from(2));
        number /= BigInt::from(2);
    }

    let mut factor: BigInt = BigInt::from(3);
    while &factor * &factor <= number {
        while &number % &factor == BigInt::zero() {
            factors.push(factor.clone());
            number /= &factor;
        }
        factor += BigInt::from(2);
    }

    if number > BigInt::from(2) {
        factors.push(number);
    }

    factors
}

pub fn contains_only_single_digit_factors(factors: Vec<BigInt>) -> bool {
    !factors.par_iter().any(|factor: &BigInt| *factor > BigInt::from(9))
}