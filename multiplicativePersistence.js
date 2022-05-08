function product(input) {
	let result = 1;

	// get each digit by mod instead of string conversion
	while (input > 0) {
		result *= input % 10;
		input /= 10;
	}

	return result;
}

function multiplicativePersistence(userInput) {
	let steps = 0;

	// 10 is smallest double digit number
	while (userInput >= 10) {
		userInput = product(userInput);
		steps++;
	}

	return steps;
}

// largest step count discovered = 277777788888899

let highestStepsCount = 0;
let highestStepsNumber = 0;

let start = 5740000000000;
let finish = 1000000000000000;

for (let i = start; i < finish; i++) {
	// console.log(`${i}: ${multiplicativePersistence(i)}`);
	if (multiplicativePersistence(i) > highestStepsCount) {
		highestStepsCount = multiplicativePersistence(i);
		highestStepsNumber = i;
	}
	if (i % 1000000 == 0) {
		console.log(`Upto ${i} so far: ${highestStepsNumber}`);
	}
}

console.log(`Highest step count: ${highestStepsNumber} at ${highestStepsCount}`);