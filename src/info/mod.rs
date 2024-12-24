use crate::utils::{set_default_colour, set_random_colour};
use sysinfo::System;

mod cpu;
mod disk;
mod kernel;
mod mem;
mod os;
mod swap;
mod virt;
pub fn run_info() {
    let s = System::new_all();
    set_random_colour();
    println!("OS  : {}", os::get_os());
    set_random_colour();
    println!("{}", cpu::get_cpu(&s));
    set_random_colour();
    println!("MEM : {}", mem::get_mem(&s));
    set_random_colour();
    for disk in disk::get_disk().iter() {
        set_random_colour();
        println!("DISK: {}", disk);
    }

    #[cfg(target_os = "linux")]
    {
        set_random_colour();
        println!("SWAP: {}", swap::get_swap());
        set_random_colour();
        println!("KERN: {}", kernel::get_kernel());
        set_random_colour();
        println!("VIRT: {}", virt::get_virt());
    }

    set_default_colour();
    println!();
}
