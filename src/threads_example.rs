use aes::Aes128;
use cipher::{BlockEncrypt, KeyInit};
use interoptopus::ffi_function;
use std::thread;
use std::time::Instant;
use tokio::runtime::Runtime;
use tokio::task;

async fn async_benchmark() -> f64 {
    const ITERATIONS: usize = 100_000;
    const THREADS: usize = 8;

    // AES encryption key and data
    let key = [0u8; 16];
    let data = [0u8; 16];

    // Start the clock
    let start = Instant::now();

    // Spawn multiple tasks to perform the calculations concurrently
    let mut tasks = vec![];
    for _ in 0..THREADS {
        let key = key;
        let data = data;
        tasks.push(task::spawn(async move {
            let cipher = Aes128::new(&key.into());
            let block = data;
            for _ in 0..(ITERATIONS / THREADS) {
                cipher.encrypt_block(&mut block.into());
            }
        }));
    }

    // Wait for all tasks to complete
    for task in tasks {
        task.await.unwrap();
    }

    // Stop the clock
    let end = Instant::now();

    //println!("Rust (async with threads and AES encryption): {} seconds", (end - start).as_secs_f64());
    (end - start).as_secs_f64()
}

#[ffi_function]
#[no_mangle]
pub extern "C" fn benchmark_rust_async() -> f64 {
    // Create a new thread to run the async function
    let handle = thread::spawn(|| Runtime::new().unwrap().block_on(async_benchmark()));

    // Wait for the thread to finish
    handle.join().unwrap()
}
