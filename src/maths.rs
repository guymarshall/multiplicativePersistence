pub fn product(mut number: i64) -> i64 {
    let mut result: i64 = 1;

    // get each digit by mod instead of string conversion
    while number > 0 {
        result *= number % 10;
        number /= 10;
    }

    result
}

pub fn multiplicative_persistence(mut number: i64) -> i32 {
    let mut steps: i32 = 0;

    // 10 is the smallest double-digit number
    while number > 10 {
        number = product(number);
        steps += 1;
    }

    steps
}

pub fn get_prime_factors(mut number: i64) -> Vec<i64> {
    let mut factors: Vec<i64> = vec![];

    while &number % 2 == 0 {
        factors.push(2);
        number /= 2;
    }

    let mut factor: i64 = 3;
    while factor * factor <= number {
        while number % factor == 0 {
            factors.push(factor);
            number /= &factor;
        }
        factor += 2;
    }

    if number > 2 {
        factors.push(number);
    }

    factors
}

pub fn contains_only_single_digit_factors(factors: Vec<i64>) -> bool {
    factors.iter().all(|factor: &i64| *factor < 10)
}
