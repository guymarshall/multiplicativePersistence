import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.Scanner;

public class FileReader {
    public static ArrayList<Long> readNumbers(String filename) throws IOException {
        Scanner scanner = new Scanner(new File(filename));
        ArrayList<Long> list = new ArrayList<>();

        while (scanner.hasNext()) {
            list.add(Long.valueOf(scanner.next()));
        }

        scanner.close();

        return list;
    }
}
