use sysinfo::System;
mod cpu;
mod disk;
mod mem;
mod os;
mod virt;
pub fn run_info() {
    let s = System::new_all();
    println!("{}", cpu::get_cpu(&s));
    println!("MEM : {}", mem::get_mem(&s));
    for disk in disk::get_disk().iter() {
        println!("DISK: {}", disk);
    }
    #[cfg(target_os = "linux")]
    {
        println!("VIRT: {}", virt::get_virt());
    }
    println!("OS  : {}", os::get_os());

}
