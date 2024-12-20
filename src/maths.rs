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
    while number >= 10 {
        number = product(number);
        steps += 1;
    }

    steps
}

pub fn contains_only_single_digit_factors(mut number: i64) -> bool {
    let mut factor: i64 = 3;
    while factor * factor <= number && factor < 8 {
        while number % factor == 0 {
            number /= &factor;
        }

        factor += if factor == 2 { 1 } else { 2 };
    }

    if number > 7 {
        return false;
    }

    true
}
