#[allow(dead_code)]
pub fn get_kernel() -> String {
    match std::fs::read_to_string("/proc/sys/kernel/osrelease") {
        Ok(kernel) => kernel.trim().to_string(),
        Err(_) => "Error - Please report to issues.".to_string(),
    }
}
