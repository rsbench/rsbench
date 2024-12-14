use sysinfo::System;
pub fn get_os() -> String {
    let os_name = match System::name() {
        Some(os_name) => os_name,
        None => "Unknown".to_string(),
    };
    let os_ver = match System::os_version() {
        Some(os_ver) => os_ver,
        None => "Unknown".to_string(),
    };
    format!("{} {}", os_name, os_ver)
}
