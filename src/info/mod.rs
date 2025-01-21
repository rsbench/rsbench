use crate::utils::color::{set_colour, set_default_colour};
use crate::GLOBAL_STRING;
use crate::{global_print, global_println};
use std::fmt::Write;
use sysinfo::System;
use termcolor::Color;

mod cpu;
mod disk;
mod kernel;
mod mem;
mod os;
mod swap;
mod virt;
pub fn run_info() {
    let s = System::new_all();

    println!();
    global_println!();

    set_colour(Color::Yellow);
    println!("INFO:");
    global_println!("INFO:");

    set_colour(Color::Yellow);
    print!("OS  : ");
    global_print!("OS  : ");
    set_colour(Color::Cyan);
    println!("{}", os::get_os());
    global_println!("{}", os::get_os());

    set_colour(Color::Yellow);
    print!("CPU : ");
    global_print!("CPU : ");
    set_colour(Color::Cyan);
    println!("{}", cpu::get_cpu().unwrap_or_default());
    global_println!("{}", cpu::get_cpu().unwrap_or_default());

    set_colour(Color::Yellow);
    print!("MEM : ");
    global_print!("MEM : ");
    set_colour(Color::Cyan);
    println!("{}", mem::get_mem(&s));
    global_println!("{}", mem::get_mem(&s));

    for disk in &disk::get_disk() {
        set_colour(Color::Yellow);
        print!("DISK: ");
        global_print!("DISK: ");
        set_colour(Color::Cyan);
        println!("{disk}");
        global_println!("{disk}");
    }

    #[cfg(target_os = "linux")]
    {
        set_colour(Color::Yellow);
        print!("SWAP: ");
        global_print!("SWAP: ");
        set_colour(Color::Cyan);
        println!("{}", swap::get_swap());
        global_println!("{}", swap::get_swap());

        set_colour(Color::Yellow);
        print!("KERN: ");
        global_print!("KERN: ");
        set_colour(Color::Cyan);
        println!("{}", kernel::get_kernel());
        global_println!("{}", kernel::get_kernel());

        set_colour(Color::Yellow);
        print!("VIRT: ");
        global_print!("VIRT: ");
        set_colour(Color::Cyan);
        println!("{}", virt::get_virt());
        global_println!("{}", virt::get_virt());
    }

    set_default_colour();
    println!();
    global_println!();
}
