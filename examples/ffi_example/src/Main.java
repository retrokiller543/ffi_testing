import static fuck.Vec3.benchmarkJava;
import static fuck.Vec3Jni.benchmarkRust;
import static fuck.Vec3JniV2.benchmarkRustV2;

class Hate {
    public static void main(String[] args) {
        benchmarkRust();
        benchmarkRustV2();
        benchmarkJava();
    }
}