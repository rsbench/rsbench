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
