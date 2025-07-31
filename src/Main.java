import java.math.BigInteger;

public class Main {
    public static void main(String[] args) {
        // 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

        final BigInteger START = new BigInteger("277780473000000");
        final BigInteger END = new BigInteger("27777778888889900000");

        for (BigInteger i = START; i.compareTo(END) < 0; i = i.add(BigInteger.ONE)) {
            if (i.mod(BigInteger.valueOf(1_000_000)).equals(BigInteger.ZERO)) {
                System.out.println(i);
            }
            int multiplicativePersistence = Maths.multiplicativePersistence(i);

            if (multiplicativePersistence > 10) {
                System.out.printf("%d has a multiplicative persistence of at least 11! Checking factors...%n", i);

                File.saveToFile("11.txt", i);

                boolean factorsAreAllSingleDigits = Maths.containsOnlySingleDigitFactors(i);

                if (factorsAreAllSingleDigits) {
                    System.out.println(i);
                    System.exit(0);
                }
            }
        }
    }
}