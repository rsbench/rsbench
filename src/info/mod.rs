use crate::utils::{set_colour, set_default_colour};
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

    set_colour(Color::Yellow);
    print!("OS  : ");
    set_colour(Color::Cyan);
    println!("{}", os::get_os());

    set_colour(Color::Yellow);
    print!("CPU : ");
    set_colour(Color::Cyan);
    println!("{}", cpu::get_cpu().unwrap_or_default());

    set_colour(Color::Yellow);
    print!("MEM : ");
    set_colour(Color::Cyan);
    println!("{}", mem::get_mem(&s));

    for disk in &disk::get_disk() {
        set_colour(Color::Yellow);
        print!("DISK: ");
        set_colour(Color::Cyan);
        println!("{disk}");
    }

    #[cfg(target_os = "linux")]
    {
        set_colour(Color::Yellow);
        print!("SWAP: ");
        set_colour(Color::Cyan);
        println!("{}", swap::get_swap());

        set_colour(Color::Yellow);
        print!("KERN: ");
        set_colour(Color::Cyan);
        println!("{}", kernel::get_kernel());

        set_colour(Color::Yellow);
        print!("VIRT: ");
        set_colour(Color::Cyan);
        println!("{}", virt::get_virt());
    }

    set_default_colour();
    println!();
}
