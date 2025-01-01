use crate::utils::{set_colour, set_default_colour};
use paris::error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use sysinfo::Disks;
use termcolor::Color;

#[cfg(not(target_os = "windows"))]
fn get_space_left() -> (f64, bool) {
    // GB, HDD / SSD
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let disk_mount_point = match disk.mount_point().to_str() {
            None => continue,
            Some(mount_point) => mount_point,
        };
        if disk_mount_point == "/tmp" {
            let kind = disk.kind().to_string();
            return if kind == "SSD" {
                (disk.available_space() as f64 / 1000000000.0, true)
            } else {
                (disk.available_space() as f64 / 1000000000.0, false)
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
                (disk.available_space() as f64 / 1000000000.0, true)
            } else {
                (disk.available_space() as f64 / 1000000000.0, false)
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
        error!("Not enough space left on disk for disk benchmark");
        return Err("Not enough space left on disk for disk benchmark".to_string());
    }

    let buffer_size: u64 = if is_ssd {
        1024 * 1024 * 1024 * 5 // 5GB
    } else {
        1024 * 1024 * 512 // 512MB
    };

    let buffer = vec![0u8; buffer_size as usize];
    let mut test_file = if let Ok(file) = File::create(set_file_path()) {
        file
    } else {
        error!("Unable to create test file");
        return Err("Unable to create test file".to_string());
    };

    let start = std::time::Instant::now();

    if let Ok(()) = test_file.write_all(&buffer) {
    } else {
        error!("Unable to write to test file");
        return Err("Unable to write to test file".to_string());
    };

    if let Ok(()) = test_file.sync_all() {
    } else {
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

    let mut test_file = if let Ok(file) = File::open(set_file_path()) {
        file
    } else {
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
        let bytes_read = if let Ok(bytes_read) = test_file.read(&mut buffer) {
            bytes_read
        } else {
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
fn set_file_path() -> &'static Path {
    Path::new("/tmp/rsbench_disk_test")
}

#[cfg(target_os = "windows")]
fn set_file_path() -> &'static Path {
    Path::new("C:\\rsbench_disk_test")
}

pub fn run_disk_speed_test() {
    let (disk_write, is_ssd) = match write_disk_test() {
        Ok((disk_write, is_ssd)) => (disk_write, is_ssd),
        Err(_) => {
            return;
        }
    };

    let disk_read = match read_disk_test(is_ssd) {
        Ok(disk_read) => disk_read,
        Err(_) => {
            return;
        }
    };

    set_colour(Color::Yellow);
    print!("DISK: ");
    set_colour(Color::Rgb(175, 231, 154));
    print!("{disk_write:.2} MB/s");
    set_colour(Color::Yellow);
    print!(" | ");
    set_colour(Color::Rgb(118, 26, 160));
    println!("{disk_read:.2} MB/s");
    set_default_colour();
}
