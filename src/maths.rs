use rug::Integer;

fn product(mut number: Integer) -> Integer {
    let mut result: Integer = Integer::from(1);

    // get each digit by mod instead of string conversion
    while number > 0 {
        result *= Integer::from(&number % 10);
        number /= 10;
    }

    result
}

pub(crate) fn multiplicative_persistence(mut number: Integer) -> i8 {
    let mut steps: i8 = 0;

    // 10 is the smallest double-digit number
    while number >= 10 {
        number = product(number);
        steps += 1;
    }

    steps
}
