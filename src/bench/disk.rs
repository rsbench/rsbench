use crate::utils::color::{set_colour, set_default_colour};
use crate::utils::term::process_decimal_point;
use crate::GLOBAL_STRING;
use crate::{global_print, global_println};
use paris::error;
use std::fmt::Write;
use std::fs;
use std::fs::File;
use std::io::{Read, Write as OtherWrite};
use std::path::PathBuf;
use std::time::Instant;
use termcolor::Color;

const TOTAL_SIZE: usize = 1024 * 1024 * 1024 * 3; // 3GB

#[cfg(not(target_os = "windows"))]
fn get_space_left() -> f64 {
    use sysinfo::Disks;

    let binding = std::env::temp_dir();
    let system_tmp_dir = binding.to_str().unwrap();
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let disk_mount_point = match disk.mount_point().to_str() {
            None => continue,
            Some(mount_point) => mount_point,
        };
        if disk_mount_point == system_tmp_dir {
            return disk.available_space() as f64 / 1_000_000_000.0;
        }
        if disk_mount_point == "/tmp" {
            return disk.available_space() as f64 / 1_000_000_000.0;
        }
        if disk_mount_point == "/" {
            return disk.available_space() as f64 / 1_000_000_000.0;
        }
    }

    error!("Unable to find root / tmp disk");
    0.0
}

#[cfg(target_os = "windows")]
fn get_space_left() -> f64 {
    114514.0
}

pub fn write_disk_test() -> Result<f64, String> {
    let mut log = paris::Logger::new();
    log.loading("Running disk write speed benchmark...");

    delete_test_file();

    let space_left = get_space_left();

    if space_left < 3.5 {
        log.done();
        error!("Not enough space left on disk for disk benchmark");
        return Err("Not enough space left on disk for disk benchmark".to_string());
    }

    let Ok(mut test_file) = File::create(set_file_path()) else {
        log.done();
        error!("Unable to create test file");
        return Err("Unable to create test file".to_string());
    };

    let chunk_size = 1 * 1024 * 1024; // 1MB

    let start_time = Instant::now();
    let mut written_bytes = 0;

    while written_bytes < TOTAL_SIZE {
        let chunk_size = std::cmp::min(chunk_size, TOTAL_SIZE - written_bytes);
        let data: Vec<u8> = (0..chunk_size).map(|_| 0).collect();
        if let Ok(_) = test_file.write_all(&data) {
            written_bytes += chunk_size;
        } else {
            log.done();
            error!("Unable to write to test file");
            return Err("Unable to write to test file".to_string());
        };
    }

    if let Err(_) = test_file.sync_all() {
        log.done();
        error!("Unable to sync test file");
        return Err("Unable to sync test file".to_string());
    };

    let elapsed_time = start_time.elapsed();
    let elapsed_seconds = elapsed_time.as_secs_f64();

    let speed = (TOTAL_SIZE as f64 / elapsed_seconds) / (1024.0 * 1024.0); // MB/s

    log.done();

    Ok(speed)
}

pub fn read_disk_test() -> Result<f64, String> {
    let mut log = paris::Logger::new();
    log.loading("Running disk read speed benchmark...");

    let Ok(mut test_file) = File::open(set_file_path()) else {
        log.done();
        error!("Unable to open test file");
        return Err("Unable to open test file".to_string());
    };

    let mut buffer = vec![0u8; 1024 * 1024]; // 1MB
    let mut total_read = 0;

    let start = std::time::Instant::now();

    while total_read < TOTAL_SIZE {
        let chunk_size = std::cmp::min(1024 * 1024, TOTAL_SIZE - total_read);
        let bytes_read = match test_file.read(&mut buffer[..chunk_size]) {
            Ok(bytes) => bytes,
            Err(_) => {
                log.done();
                error!("Unable to read from test file");
                return Err("Unable to read from test file".to_string());
            }
        };
        if bytes_read == 0 {
            break;
        }
        total_read += bytes_read;
    }

    let elapsed_time = start.elapsed();
    let elapsed_seconds = elapsed_time.as_secs_f64();
    let speed = (TOTAL_SIZE as f64 / elapsed_seconds) / (1024.0 * 1024.0); // MB/s
    log.done();

    delete_test_file();

    Ok(speed)
}

fn delete_test_file() {
    let _ = fs::remove_file(set_file_path());
}

#[cfg(not(target_os = "windows"))]
fn set_file_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("rsbench_disk_test");
    path
}

#[cfg(target_os = "windows")]
fn set_file_path() -> PathBuf {
    use std::str::FromStr;
    PathBuf::from_str("C:\\rsbench_disk_test").unwrap()
}

pub fn run_disk_speed_test() {
    let Ok(disk_write) = write_disk_test() else {
        return;
    };

    let Ok(disk_read) = read_disk_test() else {
        return;
    };

    let binding = disk_write.to_string();
    let disk_write = process_decimal_point(&binding.as_str()[0..6]);

    let binding = disk_read.to_string();
    let disk_read = process_decimal_point(&binding.as_str()[0..6]);

    set_colour(Color::Yellow);
    print!("DISK: ");
    global_print!("DISK: ");
    set_colour(Color::Rgb(175, 231, 154));
    print!("{disk_write} MB/s");
    global_print!("{disk_write} MB/s");
    set_colour(Color::Yellow);
    print!(" | ");
    global_print!(" | ");
    set_colour(Color::Rgb(118, 26, 160));
    println!("{disk_read} MB/s");
    global_println!("{disk_read} MB/s");
    set_default_colour();
}
