import java.math.BigInteger;

public final class Main {
    public static void main(String[] args) {
        // 0, 10, 25, 39, 77, 679, 6788, 68889, 2677889, 26888999, 3778888999, 277777788888899

        final BigInteger START = new BigInteger("277909186000000");
        final BigInteger END = new BigInteger("27777778888889900000");

        for (BigInteger i = START; i.compareTo(END) < 0; i = i.add(BigInteger.ONE)) {
            if (i.mod(BigInteger.valueOf(1_000_000)).equals(BigInteger.ZERO)) {
                System.out.println(i);
            }

            if (Maths.containsZero(i)) {
                continue;
            }

            if (Maths.containsFive(i)) {
                continue;
            }

            final int MULTIPLICATIVE_PERSISTENCE = Maths.multiplicativePersistence(i);

            if (MULTIPLICATIVE_PERSISTENCE > 10) {
                System.out.printf("%d has a multiplicative persistence of at least 11! Checking factors...%n", i);

                File.saveToFile("11.txt", i);

                final boolean FACTORS_ARE_ALL_SINGLE_DIGITS = Maths.containsOnlySingleDigitFactors(i);

                if (FACTORS_ARE_ALL_SINGLE_DIGITS) {
                    System.out.println(i);
                    System.exit(0);
                }
            }
        }
    }
}