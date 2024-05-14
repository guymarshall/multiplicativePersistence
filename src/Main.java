import java.util.ArrayList;

// 31262221321885653647626425815737111943458241183262682285162767533715254994149452428386848427865279586287233185834769954797571297422117699584

public class Main {
    public static void main(String[] args) {
        double startTime = System.nanoTime();

        int count = 10;
        long number = 277777788888899L;
        ArrayList<Long> numbersWithPersistenceEleven = new ArrayList<>(count);

        while (count > 0) {
            long multiplicativePersistence = Maths.multiplicativePersistence(number);

            if (multiplicativePersistence == 11) {
                System.out.printf("Numbers left: %d%n", count);
                numbersWithPersistenceEleven.add(number);
                count--;

                if (Maths.containsOnlySingleDigitFactors(Maths.getPrimeFactors(number))) {
                    System.out.println(number);
                }
            }

            number++;
        }

        System.out.println(numbersWithPersistenceEleven);

        double endTime = System.nanoTime();

        double elapsedTime = (endTime - startTime) / 1_000_000_000.0;

        System.out.printf("Elapsed time: %f seconds%n", elapsedTime);
    }
}