use sysinfo::Disks;
pub fn get_disk() -> Vec<String> {
    let d = Disks::new_with_refreshed_list();
    let mut diskinfo = Vec::new();
    for disk in &d {
        #[cfg(target_os = "linux")]
        {
            // Skip non-devices
            if !disk.name().to_str().unwrap().starts_with("/dev/"){
                continue;
            };
            // Skip boot partitions and unmounted partitions
            match disk.mount_point().to_str() {
                None => continue,
                Some(mount_point) => {
                    if mount_point.starts_with("/boot") {
                        continue;
                    }
                }
            }
        }

        /* For some weird reason, memory seems to be calculated entirely differently than storage in sysinfo.
           Disk is *lobytes while memory is *bibytes
           IDK why but whatever
        */

        if disk.total_space() > 1_000_000_000 {
            /* Shouldn't be needed anymore
            #[cfg(target_os = "linux")]
            {
                /*  why tf is there a random 1GB partition for some linux systems? Only god knows
                    probably just a swap or boot partition
                    either way, no way that's going to be used
                    gonna skip it until I can figure out a better way
                */
                if disk.total_space() < 2_000_000_000 {
                    continue;
                }
            }
            */
            diskinfo.push(format!(
                "{}/{} GB",
                (disk.total_space() - disk.available_space()) / 1_000_000_000,
                disk.total_space() / 1_000_000_000
            ));
        } else {
            diskinfo.push(format!(
                "{}/{} MB",
                (disk.total_space() - disk.available_space()) / 1_000_000,
                disk.total_space() / 1_000_000
            ));
        }
    }
    diskinfo
}
