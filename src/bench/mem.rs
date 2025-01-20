use crate::utils::color::{set_colour, set_default_colour};
use crate::GLOBAL_STRING;
use crate::{global_print, global_println};
use paris::error;
use std::fmt::Write;
use std::hint::black_box;
use std::ptr::null_mut;
use std::time::Instant;
use termcolor::Color;

#[cfg(not(target_os = "windows"))]
fn get_avaliable_memory() -> f64 {
    // MB
    let mut s = sysinfo::System::new();
    s.refresh_all();
    let memory = s.available_memory();
    memory as f64 / 1024.0 / 1024.0
}

#[cfg(not(target_os = "windows"))]
fn mem_test() -> (f64, f64) {
    use libc::{c_void, mmap, munmap, MAP_ANON, MAP_PRIVATE, PROT_READ, PROT_WRITE};

    let available_memory = get_avaliable_memory();
    let memory_size = if available_memory < 100.0 {
        available_memory * 1024.0 * 1024.0 * 0.8
    } else {
        1024.0 * 1024.0 * 100.0 // 100MB
    };

    let memory_size = memory_size as usize;

    let memory: *mut u8 = unsafe {
        mmap(
            null_mut(),             // 让系统选择映射地址
            memory_size,            // 映射的大小
            PROT_READ | PROT_WRITE, // 可读可写
            MAP_ANON | MAP_PRIVATE, // 匿名映射，私有
            -1,                     // 文件描述符（匿名映射时为 -1）
            0,                      // 偏移量
        )
        .cast::<u8>()
    };

    if memory.is_null() {
        error!("Failed to allocate memory with mmap");
        global_println!("❌ Failed to allocate memory with mmap");
        return (0.0, 0.0);
    }

    // 测试写入速度
    let start_write = Instant::now();
    unsafe {
        for i in 0..memory_size {
            *(memory.add(i)) = (i % 256) as u8;
        }
    }
    let write_duration = start_write.elapsed();
    let write_speed = (memory_size as f64 / 1024.0 / 1024.0) / write_duration.as_secs_f64();

    // 测试读取速度
    let start_read = Instant::now();
    let mut sum = 0u8;
    unsafe {
        for i in 0..memory_size {
            sum = sum.wrapping_add(*(memory.add(i)));
        }
    }
    let read_duration = start_read.elapsed();
    let read_speed = (memory_size as f64 / 1024.0 / 1024.0) / read_duration.as_secs_f64();

    // 防止编译器优化掉读取操作
    black_box(sum);

    // 释放内存
    unsafe {
        munmap(memory.cast::<c_void>(), memory_size);
    }
    (write_speed, read_speed)
}

#[cfg(target_os = "windows")]
fn mem_test() -> (f64, f64) {
    use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};
    use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, MEM_RESERVE, PAGE_READWRITE};

    const MEMORY_SIZE: usize = 1024 * 1024 * 100; // 100 MB

    // 使用 VirtualAlloc 分配内存
    let memory: *mut u8 = unsafe {
        VirtualAlloc(
            null_mut(),
            MEMORY_SIZE,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        ) as *mut u8
    };

    if memory.is_null() {
        error!("Failed to allocate memory with VirtualAlloc");
        global_println!("❌ Failed to allocate memory with VirtualAlloc");
        return (0.0, 0.0);
    }

    let start_write = Instant::now();
    unsafe {
        for i in 0..MEMORY_SIZE {
            *(memory.add(i)) = (i % 256) as u8;
        }
    }
    let write_duration = start_write.elapsed();
    let write_speed = (MEMORY_SIZE as f64 / 1024.0 / 1024.0) / write_duration.as_secs_f64();

    // 测试读取速度
    let start_read = Instant::now();
    let mut sum = 0u8;
    unsafe {
        for i in 0..MEMORY_SIZE {
            sum = sum.wrapping_add(*(memory.add(i)));
        }
    }
    let read_duration = start_read.elapsed();
    let read_speed = (MEMORY_SIZE as f64 / 1024.0 / 1024.0) / read_duration.as_secs_f64();

    // 防止编译器优化掉读取操作
    black_box(sum);

    // 释放内存
    unsafe {
        VirtualFree(memory as *mut _, 0, MEM_RELEASE);
    }
    (write_speed, read_speed)
}

pub fn run_men_test() {
    let (write_speed, read_speed) = mem_test();

    let binding = write_speed.to_string();
    let write_speed = &binding.as_str()[0..6];

    let binding = read_speed.to_string();
    let read_speed = &binding.as_str()[0..6];

    set_colour(Color::Yellow);
    print!("MEM : ");
    global_print!("MEM : ");
    set_colour(Color::Rgb(250, 91, 122));
    print!("{write_speed} MB/s");
    global_print!("{write_speed} MB/s");
    set_colour(Color::Yellow);
    print!(" | ");
    global_print!(" | ");
    set_colour(Color::Rgb(59, 124, 63));
    println!("{read_speed} MB/s");
    global_println!("{read_speed} MB/s");
    set_default_colour();
}
