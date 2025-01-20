use crate::utils::color::{set_colour, set_default_colour};
use paris::error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::str::FromStr;
use termcolor::Color;

#[cfg(not(target_os = "windows"))]
fn get_space_left() -> (f64, bool) {
    use std::env;
    use sysinfo::Disks;

    let system_tmp_dir = env::var("TMPDIR").unwrap_or("/tmp".to_string());
    // GB, HDD / SSD
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let disk_mount_point = match disk.mount_point().to_str() {
            None => continue,
            Some(mount_point) => mount_point,
        };
        if disk_mount_point == system_tmp_dir {
            let kind = disk.kind().to_string();
            return if kind == "SSD" {
                (disk.available_space() as f64 / 1_000_000_000.0, true)
            } else {
                (disk.available_space() as f64 / 1_000_000_000.0, false)
            };
        }
        if disk_mount_point == "/tmp" {
            let kind = disk.kind().to_string();
            return if kind == "SSD" {
                (disk.available_space() as f64 / 1_000_000_000.0, true)
            } else {
                (disk.available_space() as f64 / 1_000_000_000.0, false)
            };
        }
    }

    for disk in &disks {
        let disk_mount_point = match disk.mount_point().to_str() {
            None => continue,
            Some(mount_point) => mount_point,
        };
        if disk_mount_point == "/" {
            let kind = disk.kind().to_string();
            return if kind == "SSD" {
                (disk.available_space() as f64 / 1_000_000_000.0, true)
            } else {
                (disk.available_space() as f64 / 1_000_000_000.0, false)
            };
        }
    }

    error!("Unable to find root / tmp disk");
    (0.0, false)
}

#[cfg(target_os = "windows")]
fn get_space_left() -> (f64, bool) {
    // GB, HDD / SSD
    (114514.0, false) // Default allow
}

pub fn write_disk_test() -> Result<(f64, bool), String> {
    let mut log = paris::Logger::new();
    log.loading("Running disk write speed benchmark...");

    delete_test_file();

    let (space_left, is_ssd) = get_space_left();

    if (is_ssd && space_left < 5.0) || (!is_ssd && space_left < 1.0) {
        log.done();
        error!("Not enough space left on disk for disk benchmark");
        return Err("Not enough space left on disk for disk benchmark".to_string());
    }

    let buffer_size: u64 = if is_ssd {
        1024 * 1024 * 1024 * 5 // 5GB
    } else {
        1024 * 1024 * 512 // 512MB
    };

    let buffer = vec![0u8; buffer_size as usize];
    let Ok(mut test_file) = File::create(set_file_path()) else {
        log.done();
        error!("Unable to create test file");
        return Err("Unable to create test file".to_string());
    };

    let start = std::time::Instant::now();

    if let Ok(()) = test_file.write_all(&buffer) {
    } else {
        log.done();
        error!("Unable to write to test file");
        return Err("Unable to write to test file".to_string());
    };

    if let Ok(()) = test_file.sync_all() {
    } else {
        log.done();
        error!("Unable to sync test file");
        return Err("Unable to sync test file".to_string());
    }

    let write_time = start.elapsed();
    log.done();

    let speed = buffer_size as f64 / 1024.0 / 1024.0 / write_time.as_secs_f64();

    Ok((speed, is_ssd))
}

pub fn read_disk_test(is_ssd: bool) -> Result<f64, String> {
    let mut log = paris::Logger::new();
    log.loading("Running disk read speed benchmark...");

    let Ok(mut test_file) = File::open(set_file_path()) else {
        log.done();
        error!("Unable to open test file");
        return Err("Unable to open test file".to_string());
    };

    let mut buffer = vec![0u8; 1024 * 1024]; // 1MB
    let mut total_read = 0;
    let file_size: u64 = if is_ssd {
        1024 * 1024 * 1024 * 5 // 5GB
    } else {
        1024 * 1024 * 512 // 512MB
    };

    let start = std::time::Instant::now();

    while total_read < file_size * 1024 * 1024 {
        let Ok(bytes_read) = test_file.read(&mut buffer) else {
            log.done();
            error!("Unable to read from test file");
            return Err("Unable to read from test file".to_string());
        };
        if bytes_read == 0 {
            break;
        }
        total_read += bytes_read as u64;
    }

    let read_time = start.elapsed();
    log.done();
    let speed = total_read as f64 / 1024.0 / 1024.0 / read_time.as_secs_f64();

    delete_test_file();
    Ok(speed)
}

fn delete_test_file() {
    let _ = fs::remove_file(set_file_path());
}

#[cfg(not(target_os = "windows"))]
fn set_file_path() -> PathBuf {
    use std::env;
    let system_tmp_dir = env::var("TMPDIR").unwrap_or("/tmp".to_string());

    PathBuf::from_str(&format!("{system_tmp_dir}/rsbench_disk_test"))
        .unwrap_or(PathBuf::from_str("./rsbench_disk_test").unwrap())
}

#[cfg(target_os = "windows")]
fn set_file_path() -> PathBuf {
    PathBuf::from_str("C:\\rsbench_disk_test").unwrap()
}

pub fn run_disk_speed_test() {
    let Ok((disk_write, is_ssd)) = write_disk_test() else {
        return;
    };

    let Ok(disk_read) = read_disk_test(is_ssd) else {
        return;
    };

    let binding = disk_write.to_string();
    let disk_write = &binding.as_str()[0..6];

    let binding = disk_read.to_string();
    let disk_read = &binding.as_str()[0..6];

    set_colour(Color::Yellow);
    print!("DISK: ");
    set_colour(Color::Rgb(175, 231, 154));
    print!("{disk_write} MB/s");
    set_colour(Color::Yellow);
    print!(" | ");
    set_colour(Color::Rgb(118, 26, 160));
    println!("{disk_read} MB/s");
    set_default_colour();
}
