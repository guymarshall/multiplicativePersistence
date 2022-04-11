use std::collections::HashMap;

fn multiplicative_persistence(user_input: i128) -> i128 {
	let mut steps: i128 = 0;
	let mut numbers: Vec<i128> = Vec::new();
	while user_input > 10 {
		let string_number: String = user_input.to_string();
		let digits: Vec<&str> = string_number.split("").collect();
		let mut sum: i128 = 1;
		let digits_count = digits.len();

		for number in 0..=digits_count {
			sum *= digits[number].parse::<i128>().unwrap();
		}

		numbers.push(sum);

		steps += 1;
	}

	return steps;
}

fn main() {
	println!("Hello, world!");
	let _user_input: i128 = 277777788888899;

	let highest_steps: HashMap<String, i128> = HashMap::from([
		("steps".to_string(), 0),
		("number".to_string(), 0)
	]);

	let start: i128 = 77551000000;
	let finish: i128 = 1000000000000000;

	for number in start..=finish {
		println!("{}: {}", number, multiplicative_persistence(number));
		// if multiplicative_persistence(number) > highest_steps.get("steps") {
		// 	highest_steps.get("steps") = multiplicative_persistence(number);
		// 	highest_steps.get("number") += 1;
		// }
		// if number % 1000000 == 0 {
		// 	println!("Upto {} so far: {:?}", number, highest_steps.get("number"));
		// }
	}

	println!("Highest step count: {:?} at {:?}", highest_steps.get("number"), highest_steps.get("steps"));
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