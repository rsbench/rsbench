use std::sync::{Arc, Mutex};
use std::time::Instant;
const BASELINE_FIB_SCORE: f32 = 8838.0;
// intentionally unoptimized fibonacci
fn fib(n: u64) -> u64 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}
pub fn run_fibonacci() {
    let mut log = paris::Logger::new();
    log.loading("Running fibonacci benchmark...");
    let start = Instant::now();
    fib(48);
    let duration = start.elapsed();
    log.done();
    let score: f32 = BASELINE_FIB_SCORE / duration.as_millis() as f32;
    println!("FIB : {} ({}ms)", score, duration.as_millis());
}

pub fn run_fibonacci_mt(threads: u32) {
    let mut log = paris::Logger::new();
    log.loading("Running multithreaded fibonacci benchmark...");
    let durations = Arc::new(Mutex::new(Vec::new()));
    let start_all = Instant::now();
    let mut handles = vec![];

    for _ in 0..threads {
        let d = Arc::clone(&durations);
        let handle = std::thread::spawn(move || {
            let start = Instant::now();
            fib(48);
            let time_ms = start.elapsed().as_millis();
            d.lock().unwrap().push(time_ms);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let total_time = start_all.elapsed().as_millis();
    log.done();

    let durations = durations.lock().unwrap();
    let mut total_score = 0.0;
    for t in durations.iter() {
        total_score += BASELINE_FIB_SCORE / (*t as f32);
    }

    println!(
        "FIB{}: {} ({}ms total, thread times = {:?}ms each)",
        threads, total_score, total_time, durations
    );
}
