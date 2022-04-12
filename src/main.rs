// fn multiplicative_persistence(mut user_input: i128) -> i128 {
// 	let mut steps: i128 = 0;
// 	let mut numbers: Vec<i128> = Vec::new();
// 	while user_input > 10 {
// 		let string_number: String = user_input.to_string();
// 		let digits: Vec<&str> = string_number.split("").collect();
// 		let mut sum: i128 = 1;
// 		let digits_count = digits.len();

// 		for number in 1..digits_count - 1 {
// 			sum *= digits[number].parse::<i128>().unwrap();
// 		}

// 		numbers.push(sum);

// 		steps += 1;
// 		user_input = sum;
// 	}

// 	return steps;
// }

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

/*
for (let i = 77551000000; i < 1000000000000000; i++) {
	// console.log(`${i}: ${multiplicativePersistence(i).steps}`);
	if (multiplicativePersistence(i).steps > highestSteps.steps) {
		highestSteps.steps = multiplicativePersistence(i).steps;
		highestSteps.number = i;
	}
	if (i % 1000000 == 0) {
		console.log(`Upto ${i} so far: ${highestSteps.number}`);
	}
}

console.log(`Highest step count: ${highestSteps.number} at ${highestSteps.steps}`);
*/