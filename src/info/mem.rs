use sysinfo::System;
pub fn get_mem(s: &System) -> String {
    let memory = s.total_memory();
    if memory > 1_048_576 {
        format!("{} GB", memory / 1_073_741_824)
    } else {
        format!("{} MB", memory / 1_048_576)
    }
}
