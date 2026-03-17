import java.util.Random;
public class Rand {
    public static void main(String[] args) {
        var rnd = new Random();
        rnd.setSeed(1);
        System.out.print("[");
        for (int i = 0; i < 16; i++) {
            System.out.print(rnd.nextInt());
            if (i != 15) {
                System.out.print(",");
            }
        }
        System.out.println("]");

        rnd.setSeed(1);
        System.out.print("[");
        for (int i = 0; i < 16; i++) {
            System.out.print(rnd.nextLong());
            if (i != 15) {
                System.out.print(",");
            }
        }
        System.out.println("]");

        rnd.setSeed(1);
        System.out.print("[");
        for (int i = 0; i < 16; i++) {
            System.out.print(rnd.nextFloat());
            if (i != 15) {
                System.out.print(",");
            }
        }
        System.out.println("]");

        rnd.setSeed(1);
        System.out.print("[");
        for (int i = 0; i < 16; i++) {
            System.out.print(rnd.nextDouble());
            if (i != 15) {
                System.out.print(",");
            }
        }
        System.out.println("]");
    }
}
