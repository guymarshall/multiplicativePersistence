fn product(mut input: i128) -> i128 {
	let mut result: i128 = 1;
	
	// get each digit by mod instead of string conversion
	while input > 0 {
		result *= input % 10;
		input /= 10;
	}
	result
}
  
fn multiplicative_persistence(mut user_input: i128) -> i128 {
	let mut steps: i128 = 0;

	// 10 is smallest double digit number
	while user_input >= 10 {
		user_input = product(user_input);
		steps += 1;
	}
	steps
}

fn main() {
	// largest step count discovered = 277777788888899

	let mut highest_steps_count: i128 = 0;
	let mut highest_steps_number: i128 = 0;

	let start: i128 = 77551000000;
	let finish: i128 = 1000000000000000;

	for number in start..=finish {
		// println!("{}: {}", number, multiplicative_persistence(number));
		if multiplicative_persistence(number) > highest_steps_count {
			highest_steps_count = multiplicative_persistence(number);
			highest_steps_number = number;
		}
		if number % 1000000 == 0 {
			println!("Upto {} so far: {}", number, highest_steps_number);
		}
	}

	println!("Highest step count: {} at {}", highest_steps_number, highest_steps_count);
}