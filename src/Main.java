public class Main {
    public static void main(String[] args) {
        byte highest_steps_count = 0;
        long highest_steps_number = 0L;

        int start = 0;
        long finish = 277777788888899L;

        for (long i = start; i <= finish; i++) {
            byte result = Maths.multiplicative_persistence(i);

            if (result > highest_steps_count) {
                highest_steps_count = result;
                highest_steps_number = i;
                System.out.printf("Up to %d so far with %d steps%n", i, result);
            }
        }

        System.out.printf("Highest step count: %d at %d%n", highest_steps_number, highest_steps_count);
    }
}