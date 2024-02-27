public class Maths {
    private static long product(long input) {
        long result = 1;

        // get each digit by mod instead of string conversion
        while (input > 0) {
            result *= input % 10;
            input /= 10;
        }

        return result;
    }

    private static boolean isDigitPresent(long number) {
        while (number > 0) {
            if (number % 10 == (long) 0) {
                break;
            }

            number = number / 10;
        }

        return number > 0;
    }

    public static byte multiplicativePersistence(long userInput) {
        if (isDigitPresent(userInput)) {
            return 1;
        }
        byte steps = 0;

        // 10 is smallest double-digit number
        while (userInput >= 10) {
            userInput = product(userInput);
            steps += 1;
        }

        return steps;
    }
}