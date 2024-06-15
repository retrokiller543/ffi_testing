package fuck;

import java.util.ArrayList;

public class HelloWorld {
    public static native ArrayList<String> special(ArrayList<Integer> input1, int input2);
    public static native void helloWorld();

    static {
        System.loadLibrary("ffi_lib");
    }

    public static void main(String[] args) {
        special(new ArrayList<>(5), 3);
        helloWorld();
    }
}
