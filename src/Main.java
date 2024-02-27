/**
 * 245
 *      -> 2 * 4 * 5 = 40
 *      -> 4 * 0 = 0
 *      (2 steps)
 * 3762
 *      -> 3 * 7 * 6 * 2 = 252
 *      -> 2 * 5 * 2 = 20
 *      -> 2 * 0 = 0
 *      (3 steps)
 */
public class Main
{
    public static void main(String[] args)
    {
        byte highestStepsCount = 0;
        long highestStepsNumber = 0L;

        long finish = 277777788888899L;

        for (long i = 0L; i <= finish; i++)
        {
            byte result = Maths.multiplicativePersistence(i);

            if (result > highestStepsCount)
            {
                highestStepsCount = result;
                highestStepsNumber = i;
                System.out.printf("Up to %d so far with %d steps%n", i, result);
            }
        }

        System.out.printf("Highest step count: %d at %d%n", highestStepsNumber, highestStepsCount);
    }
}