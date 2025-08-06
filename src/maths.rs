use std::fmt::Write;

use rug::Integer;

pub(crate) fn multiplicative_persistence(mut number: Integer) -> i8 {
    let mut steps: i8 = 0;

    let mut buffer: String = String::with_capacity(1024);

    // TODO: use string directly instead of converting Integer to string
    while number >= 10 {
        buffer.clear();
        write!(&mut buffer, "{number}").unwrap();

        let mut product: Integer = Integer::from(1);
        for byte in buffer.bytes() {
            if byte == b'0' {
                product = Integer::from(0);
                break;
            }
            product *= byte - b'0';
        }

        number = product;
        steps += 1;
    }

    steps
}
