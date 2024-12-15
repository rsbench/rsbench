use std::time::Instant;
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
    fib(50);
    let duration = start.elapsed();
    log.done();
    println!("FIB : {}ms", duration.as_millis());
}
