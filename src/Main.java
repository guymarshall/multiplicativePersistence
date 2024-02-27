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