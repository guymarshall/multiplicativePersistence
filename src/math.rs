pub fn product(mut input: i64) -> i64 {
	let mut result: i64 = 1;
	
	// get each digit by mod instead of string conversion
	while input > 0 {
		result *= input % 10;
		input /= 10;
	}
	result
}
  
pub fn multiplicative_persistence(mut user_input: i64) -> i64 {
	let mut steps: i64 = 0;

	// 10 is smallest double digit number
	while user_input >= 10 {
		user_input = product(user_input);
		steps += 1;
	}
	steps
}

pub fn number_to_vector(number: i64) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();

    for i in 1..number + 1 {
        numbers.push(i);
    }

    return numbers;
}