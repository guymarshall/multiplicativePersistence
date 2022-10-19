pub fn product(mut input: u64) -> u64 {
	let mut result: u64 = 1;
	
	// get each digit by mod instead of string conversion
	while input > 0 {
		result *= input % 10;
		input /= 10;
	}
	result
}
  
pub fn multiplicative_persistence(mut user_input: u64) -> u64 {
	let mut steps: u64 = 0;

	// 10 is smallest double digit number
	while user_input >= 10 {
		user_input = product(user_input);
		steps += 1;
	}
	steps
}