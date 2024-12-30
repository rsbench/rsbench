use sysinfo::System;
pub fn get_os() -> String {
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_ver = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    format!("{os_name} {os_ver}")
}
