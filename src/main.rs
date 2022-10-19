mod math;

fn main() {
	// largest step count discovered = 277777788888899

	let mut highest_steps_count: u64 = 0;
	let mut highest_steps_number: u64 = 0;

	let start: u64 = 5745600000000;
	let finish: u64 = 1000000000000000;

	for number in start..=finish {
		// println!("{}: {}", number, multiplicative_persistence(number));
		let result = math::multiplicative_persistence(number);
		if result > highest_steps_count {
			highest_steps_count = result;
			highest_steps_number = number;
		}
		if number % 100000000 == 0 {
			println!("Upto {} so far: {}", number, highest_steps_number);
		}
	}

	println!("Highest step count: {} at {}", highest_steps_number, highest_steps_count);
}