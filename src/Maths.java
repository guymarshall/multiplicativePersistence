import java.math.BigInteger;
import java.util.ArrayList;

public class Maths {
    public static BigInteger product(BigInteger input) {
        BigInteger result = BigInteger.ONE;

        // get each digit by mod instead of string conversion
        while (input.compareTo(BigInteger.ZERO) > 0) {
            result = result.parallelMultiply(input.mod(BigInteger.TEN));
            input = input.divide(BigInteger.TEN);
        }

        return result;
    }

    public static int multiplicativePersistence(BigInteger userInput) {
        int steps = 0;

        // 10 is the smallest double-digit number
        while (userInput.compareTo(BigInteger.TEN) > 0) {
            userInput = product(userInput);
            steps++;
        }

        return steps;
    }

    public static ArrayList<BigInteger> getPrimeFactors(BigInteger number) {
        ArrayList<BigInteger> factors = new ArrayList<>();

        while (number.mod(BigInteger.TWO).compareTo(BigInteger.ZERO) == 0) {
            factors.add(BigInteger.TWO);
            number = number.divide(BigInteger.TWO);
        }

        BigInteger factor = BigInteger.valueOf(3);
        while (factor.parallelMultiply(factor).compareTo(number) <= 0) {
            while (number.mod(factor).compareTo(BigInteger.ZERO) == 0) {
                factors.add(factor);
                number = number.divide(factor);
            }
            factor = factor.add(BigInteger.TWO);
        }

        if (number.compareTo(BigInteger.TWO) > 0) {
            factors.add(number);
        }

        return factors;
    }

    public static boolean containsOnlySingleDigitFactors(ArrayList<BigInteger> factors) {
        return factors.stream().noneMatch(factor -> factor.compareTo(BigInteger.valueOf(9)) > 0);
    }
}
