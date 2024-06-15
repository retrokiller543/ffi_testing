package fuck;

public class Vec3 {
    int x;
    int y;
    int z;

    public Vec3(int x, int y, int z) {
        this.x = x;
        this.y = y;
        this.z = z;
    }

    public Vec3(Vec3 v) {
        this.x = v.x;
        this.y = v.y;
        this.z = v.z;
    }

    public int add(Vec3 other) {
        this.x += other.x;
        this.y += other.y;
        this.z += other.z;

        return this.x + this.y + this.z;
    }

    public int add(int x, int y, int z) {
        this.x += x;
        this.y += y;
        this.z += z;

        return this.x + this.y + this.z;
    }

    public int dot(Vec3 other) {
        return this.x * other.x + this.y * other.y + this.z * other.z;
    }

    public Vec3 cross(Vec3 other) {
        return new Vec3(this.y * other.z - this.z * other.y, this.y * other.x - this.x * other.z, this.x * other.y - this.y * other.x);
    }

    /*
    * pub fn normalize(&self) -> Vec3 {
        let len = ((self.x * self.x + self.y * self.y + self.z * self.z) as f64).sqrt();
        Vec3 {
            x: (self.x as f64 / len) as i32,
            y: (self.y as f64 / len) as i32,
            z: (self.z as f64 / len) as i32,
        }
    }
    * */

    public Vec3 normalize() {
        float len = (float) Math.sqrt(this.x * this.x + this.y * this.y + this.z * this.z);
        return new Vec3((int) (this.x / len), (int) (this.y / len), (int) (this.z / len));
    }

    public static void benchmarkJava() {
        final long iterations = 10000000;
        long start = System.nanoTime();

        for (int i = 0; i < iterations; i++) {
            Vec3 v1 = new Vec3(1, 2, 3);
            Vec3 v2 = new Vec3(4, 5, 6);

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


