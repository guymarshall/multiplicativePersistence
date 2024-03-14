#![forbid(unsafe_code)]

pub fn product(mut input: i64) -> i64 {
    let mut result: i64 = 1;

    // get each digit by mod instead of string conversion
    while input > 0 {
        result *= input % 10;
        input /= 10;
    }
    result
}

pub fn is_digit_present(mut number: i64) -> bool {
    while number > 0 {
        if number % 10 == 0 {
            return true;
        }

        number /= 10;
    }

    false
}

pub fn multiplicative_persistence(mut user_input: i64) -> i64 {
    if is_digit_present(user_input) {
        return 1;
    }
    let mut steps: i64 = 0;

    // 10 is the smallest double-digit number
    while user_input >= 10 {
        user_input = product(user_input);
        steps += 1;
    }
    steps
}