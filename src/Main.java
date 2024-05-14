import java.io.IOException;
import java.util.ArrayList;

public class Main {
    public static void main(String[] args) {
        double startTime = System.nanoTime();

        try {
            ArrayList<Long> numbersWithPersistenceEleven = FileReader.readNumbers("numbersWithPersistenceEleven.txt");

            numbersWithPersistenceEleven
                    .stream()
                    .parallel()
                    .filter(number -> Maths.containsOnlySingleDigitFactors(Maths.getPrimeFactors(number)))
                    .forEach(System.out::println);
        } catch (IOException e) {
            System.out.println(e.getMessage());
        }

        double endTime = System.nanoTime();

        double elapsedTime = (endTime - startTime) / 1_000_000_000.0;

        System.out.printf("Elapsed time: %f seconds%n", elapsedTime);
    }
}