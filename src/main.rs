mod math;

fn main() {
	let mut highest_steps_count: i64 = 0;
	let mut highest_steps_number: i64 = 0;

	let start: i64 = 0;
	let finish: i64 = 277777788888899;

	for number in start..=finish {
		// println!("{}: {}", number, multiplicative_persistence(number));
		let result: i64 = math::multiplicative_persistence(number);
		if result > highest_steps_count {
			highest_steps_count = result;
			highest_steps_number = number;
			println!("Upto {} so far with {} steps", number, result);
		}
		// if number % 100000000 == 0 {
		// 	println!("Upto {} so far: {}", number, highest_steps_number);
		// }
	}

	println!("Highest step count: {} at {}", highest_steps_number, highest_steps_count);
}