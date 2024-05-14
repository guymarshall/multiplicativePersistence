import java.util.ArrayList;

public class Maths {
    public static long product(long input) {
        long result = 1;

        // get each digit by mod instead of string conversion
        while (input > 0) {
            result *= input % 10;
            input /= 10;
        }

        return result;
    }

    public static long multiplicativePersistence(long userInput) {
        long steps = 0;

        // 10 is the smallest double-digit number
        while (userInput >= 10) {
            userInput = product(userInput);
            steps += 1;
        }

        return steps;
    }

    public static ArrayList<Long> getPrimeFactors(long number) {
        ArrayList<Long> factors = new ArrayList<>();

        while (number % 2 == 0) {
            factors.add(2L);
            number /= 2;
        }

        long factor = 3;
        while (factor * factor <= number) {
            while (number % factor == 0) {
                factors.add(factor);
                number /= factor;
            }
            factor += 2;
        }

        if (number > 2) {
            factors.add(number);
        }

        return factors;
    }

    public static boolean containsOnlySingleDigitFactors(ArrayList<Long> factors) {
        return factors.stream().noneMatch(factor -> factor > 9);
    }
}
