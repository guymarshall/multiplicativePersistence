import java.math.BigInteger;

public class Maths {
    public static BigInteger product(BigInteger number)  {
        BigInteger result = BigInteger.ONE;

        // get each digit by mod instead of string conversion
        while (number.compareTo(BigInteger.ZERO) > 0) {
            result = result.multiply(number.mod(BigInteger.TEN));
            number = number.divide(BigInteger.TEN);
        }

        return result;
    }

    public static int multiplicativePersistence(BigInteger number) {
        int steps = 0;

        // 10 is the smallest double-digit number
        while (number.compareTo(BigInteger.TEN) >= 0) {
            number = product(number);
            steps++;
        }

        return steps;
    }

    public static boolean containsOnlySingleDigitFactors(BigInteger number) {
        BigInteger factor = BigInteger.TWO;
        while (factor.multiply(factor).compareTo(number) <= 0 && factor.compareTo(BigInteger.valueOf(8)) < 0) {
            while (number.mod(factor).equals(BigInteger.ZERO)) {
                number = number.divide(factor);
            }

            if (factor.equals(BigInteger.TWO)) {
                factor = factor.add(BigInteger.ONE);
            } else {
                factor = factor.add(BigInteger.TWO);
            }
        }

        return !(number.compareTo(BigInteger.valueOf(7)) >= 0);
    }
}
