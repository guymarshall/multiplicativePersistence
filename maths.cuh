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

__device__ bool isDigitPresent(uint64_t number, uint8_t digit) {
    while (number > 0) {
        if (number % 10 == digit) {
            return true;
        }

        number = number / 10;
    }

    return false;
}

__device__ uint16_t multiplicativePersistence(uint64_t userInput) {
    if (isDigitPresent(userInput, 0)) {
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