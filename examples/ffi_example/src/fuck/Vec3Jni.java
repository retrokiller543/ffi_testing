package fuck;

public class Vec3Jni {
    public static native long vec3New(int x, int y, int z);
    public static native int vec3Add(long vecPtr, int x, int y, int z);
    public static native int vec3AddReverseArgs(long vecPtr, int x, int y, int z);
    public static native int vec3Dot(long vecPtr, long otherPtr);
    public static native long vec3Cross(long vecPtr, long otherPtr);
    public static native long vec3Normalize(long vecPtr);

    static {
        System.loadLibrary("ffi_lib");
    }

    public static void benchmarkRust() {
        final long iterations = 1;
        long start = System.nanoTime();

        for (int i = 0; i < iterations; i++) {
            long v1 = vec3New(1, 2, 3);
            long v2 = vec3New(4, 5, 6);

            vec3Add(v1, 1, 2, 3);
            vec3AddReverseArgs(v1, 1, 2, 3);
            vec3Dot(v1, v2);
            vec3Cross(v1, v2);
            vec3Normalize(v1);
        }

        long end = System.nanoTime();
        double timeSpent = (end - start) / 1e9;

        System.out.println("Rust benchmark time: " + timeSpent + " seconds");
    }
}
