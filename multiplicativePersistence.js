/*fn product(mut input: u64) -> u64 {
	let mut result: u64 = 1;
	
	// get each digit by mod instead of string conversion
	while input > 0 {
		result *= input % 10;
		input /= 10;
	}
	result
}

fn multiplicative_persistence(mut user_input: u64) -> u64 {
	let mut steps: u64 = 0;

	// 10 is smallest double digit number
	while user_input >= 10 {
		user_input = product(user_input);
		steps += 1;
	}
	steps
}

fn main() {
	// largest step count discovered = 277777788888899

	let mut highest_steps_count: u64 = 0;
	let mut highest_steps_number: u64 = 0;

	let start: u64 = 5740000000000;
	let finish: u64 = 1000000000000000;

	for number in start..=finish {
		// println!("{}: {}", number, multiplicative_persistence(number));
		if multiplicative_persistence(number) > highest_steps_count {
			highest_steps_count = multiplicative_persistence(number);
			highest_steps_number = number;
		}
		if number % 100000000 == 0 {
			println!("Upto {} so far: {}", number, highest_steps_number);
		}
	}

	println!("Highest step count: {} at {}", highest_steps_number, highest_steps_count);
}*/
// let userInput = 277777788888899;
// multiplicativePersistence(userInput);

let highestSteps = {
	'steps': 0,
	'number': 0
};

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

function splitStringIntoCharacters(input) {
	return input.split('');
}

function multiplicativePersistence(userInput) {
	let steps = 0;
	const numbers = [];
	while (userInput > 10) {
		let digits = splitStringIntoCharacters(userInput.toString());
		let sum = 1;
		for (let i = 0; i < digits.length; i++) {
			sum *= Number(digits[i]);
		}
		userInput = sum.toString();
		numbers.push(userInput);

		steps++;
	}

	return {
		'steps': steps,
		'numbers': numbers
	};
}