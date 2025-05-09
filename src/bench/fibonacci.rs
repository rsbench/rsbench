use crate::utils::color::{set_colour, set_default_colour};
use crate::GLOBAL_STRING;
use crate::{global_print, global_println};
use std::fmt::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use termcolor::Color;

const BASELINE_FIB_SCORE: f32 = 8838.0;
// intentionally unoptimized fibonacci
fn fib(n: u64) -> u64 {
    // Black Magic, if optimized, it will be BOOOOOM
    #[allow(clippy::absurd_extreme_comparisons)]
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

    set_colour(Color::Yellow);
    print!("FIB : ");
    global_print!("FIB : ");
    set_colour(Color::White);
    print!("{score}");
    global_print!("{score}");
    set_colour(Color::Magenta);
    println!(" ({}ms)", duration.as_millis());
    global_println!(" ({}ms total)", duration.as_millis());
    set_default_colour();
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

    set_colour(Color::Yellow);
    print!("FIB{threads}: ");
    global_print!("FIB{threads}: ");
    set_colour(Color::White);
    print!("{total_score}");
    global_print!("{total_score}");
    set_colour(Color::Magenta);
    println!(" ({total_time}ms total)");
    global_println!(" ({total_time}ms total)");
    set_default_colour();
}
