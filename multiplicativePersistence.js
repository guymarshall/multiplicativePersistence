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
    console.log(`Upto ${i} so far...`);
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