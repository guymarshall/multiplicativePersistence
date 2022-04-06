// get number as input
// while number not 1 digit:
  // convert to string
  // split into chars
  // convert chars into numbers
  // multiply numbers together and return total
  // add one to count

document.getElementById('button').addEventListener('click', function() {
  // let number = document.getElementById('number').value;
  // let digits = number.toString().split('');
	// console.log(digits);
  let number = document.getElementById('number').value;
  while (number > 10) {
    let digits = splitStringIntoCharacters(number);
    let sum = 1;
    for (let i = 0; i < digits.length; i++) {
      sum *= Number(digits[i]);
    }
    console.log(sum);
    number = sum.toString();
  }
});

function splitStringIntoCharacters(input) {
  return input.split('');
}