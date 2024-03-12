#include <cstdint>

__device__ uint64_t product(uint64_t input) {
    uint64_t result = 1;

    // get each digit by mod instead of string conversion
    while (input > 0) {
        result *= input % 10;
        input /= 10;
    }

    return result;
}

__device__ bool isDigitPresent(uint64_t number) {
    while (number > 0) {
        if (number % 10 == 0) {
            break;
        }

        number = number / 10;
    }

    return number > 0;
}

__device__ uint16_t multiplicativePersistence(uint64_t userInput) {
    if (isDigitPresent(userInput)) {
        return 1;
    }
    uint16_t steps = 0;

    // 10 is smallest double-digit number
    while (userInput >= 10) {
        userInput = product(userInput);
        steps += 1;
    }

    return steps;
}