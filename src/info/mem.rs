use sysinfo::System;
pub fn get_mem(s: &System) -> String {
    let memory = s.total_memory();
    if memory > 1_024_000 {
        format!("{} GB", memory / 1_024_000_000)
    } else {
        format!("{} MB", memory / 1_024_000)
    }
}
