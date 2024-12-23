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
    println!("OS  : {}", os::get_os());
    println!("{}", cpu::get_cpu(&s));
    println!("MEM : {}", mem::get_mem(&s));
    for disk in disk::get_disk().iter() {
        println!("DISK: {}", disk);
    }
    #[cfg(target_os = "linux")]
    {
        println!("SWAP: {}", swap::get_swap());
        println!("KERN: {}", kernel::get_kernel());
        println!("VIRT: {}", virt::get_virt());
    }
}
