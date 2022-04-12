fn step(mut x: i128) -> i128 {
	let mut result = 1;
	while x > 0 {
	  result *= x % 10;
	  x /= 10;
	}
	result
  }
  
  fn multiplicative_persistence(mut user_input: i128) -> i128 {
	let mut steps = 0;
	while user_input > 10 {
	  user_input = step(user_input);
	  steps += 1;
	}
	steps
  }

fn main() {
	// let _user_input: i128 = 277777788888899;

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