use sysinfo::{System, Disks};
mod cpu;
pub fn run_info() {
    //let s = System::new_with_specifics(RefreshKind::everything());
    let s = System::new_all();
    match s.cpus().first() {
        Some(cpu) => {
            let cpu_model = cpu.brand();
            let cpu_threads = s.cpus().len();
            println!("CPU : {} {} Threads", cpu_model, cpu_threads);
        }
        None => {
            println!("CPU: Unknown CPU");
        }
    }
    let memory = s.total_memory();
    if memory > 1_048_576 {
        println!("MEM : {} GB", memory / 1_073_741_824);
    } else {
        println!("MEM : {} MB", memory / 1_048_576);
    }
    let d = Disks::new_with_refreshed_list();
    for disk in &d {
        println!("DISK: {}/{} GB", (disk.total_space()-disk.available_space())/1_073_741_824, disk.total_space()/1_073_741_824);
    }
    
}
