use sysinfo::Disks;
pub fn get_disk() -> Vec<String> {
    let d = Disks::new_with_refreshed_list();
    let mut diskinfo = Vec::new();
    for disk in &d {
        #[cfg(target_os = "linux")]
        {
            // Skip non-devices
            if !disk.name().to_str().unwrap().starts_with("/dev/") {
                continue;
            }
        }

        if disk.total_space() > 1_073_741_824 {
            diskinfo.push(format!(
                "{}/{} GB",
                (disk.total_space() - disk.available_space()) / 1_073_741_824,
                disk.total_space() / 1_073_741_824
            ));
        } else {
            diskinfo.push(format!(
                "{}/{} MB",
                (disk.total_space() - disk.available_space()) / 1_048_576,
                disk.total_space() / 1_048_576
            ));
        }
    }
    diskinfo
}
