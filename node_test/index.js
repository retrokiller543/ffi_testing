import { Vec3, benchmark_rust, init_logger } from "ffi_lib";

init_logger();

let vec = Vec3.new(1, 2, 3);
console.log(vec.add(1, 2, 3));

benchmark_rust();