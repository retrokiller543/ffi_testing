package fuck;

public class Vec3JniV2 {
    private int x;
    private int y;
    private int z;

    static {
        System.loadLibrary("ffi_lib");
    }

    public Vec3JniV2(int x, int y, int z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public native int add(int x, int y, int z);
    public native int dot(Vec3JniV2 other);
    public native Vec3JniV2 cross(Vec3JniV2 other);
    public native Vec3JniV2 normalize();

    public static void benchmarkRustV2() {
        final long iterations = 1;
        long start = System.nanoTime();

        for (int i = 0; i < iterations; i++) {
            Vec3JniV2 v1 = new Vec3JniV2(1, 2, 3);
            Vec3JniV2 v2 = new Vec3JniV2(4, 5, 6);

            v1.add(1, 2, 3);
            v1.add(1, 2, 3);
            v1.dot(v2);
            v1.cross(v2);
            v1.normalize();
        }

        long end = System.nanoTime();
        double timeSpent = (end - start) / 1e9;

        System.out.println("Java benchmark time: " + timeSpent + " seconds");
    }
}
