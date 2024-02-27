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

    private static boolean is_digit_present(long number, long digit) {
        while (number > 0) {
            if (number % 10 == digit) {
                break;
            }

            number = number / 10;
        }

        return number > 0;
    }

    public static byte multiplicative_persistence(long user_input) {
        if (is_digit_present(user_input, 0)) {
            return 1;
        }
        byte steps = 0;

        // 10 is smallest double-digit number
        while (user_input >= 10) {
            user_input = product(user_input);
            steps += 1;
        }

        return steps;
    }
}