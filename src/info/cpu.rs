use sysinfo::System;
#[allow(dead_code)]
pub fn get_cpu(s: &System) -> String {
    match s.cpus().first() {
        Some(cpu) => {
            let cpu_model = cpu.brand();
            let cpu_threads = s.cpus().len();
            let cpu_speed = cpu.frequency();
            format!(
                "CPU : {} {} Threads @{}Mhz",
                cpu_model, cpu_threads, cpu_speed
            )
        }
        None => {
            format!("CPU: Unknown CPU")
        }
    }
}