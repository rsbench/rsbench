// Using sysinfo temporarily for non-linux distributions
#[cfg(not(target_os = "linux"))]
pub fn get_cpu() -> Result<String, Box<dyn std::error::Error>> {
    Ok(classic_get_cpu())
}

// Use dmidecode for linux, which produces >>>> better results especially for aarch64 processors
#[cfg(target_os = "linux")]
pub fn get_cpu() -> Result<String, Box<dyn std::error::Error>> {
    use std::env;
    if env::var("USER") == Ok("root".to_string()) {
        use dmidecode::{EntryPoint, Structure};
        use std::fs;
        let Ok(entry_point_bytes) = fs::read("/sys/firmware/dmi/tables/smbios_entry_point") else {
            return Ok(classic_get_cpu());
        };
        let Ok(dmi_table_bytes) = fs::read("/sys/firmware/dmi/tables/DMI") else {
            return Ok(classic_get_cpu());
        };

        let entry_point = EntryPoint::search(&entry_point_bytes)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        for structure_result in entry_point.structures(&dmi_table_bytes) {
            match structure_result {
                Ok(structure) => match structure {
                    Structure::Processor(cpu) => {
                        let cpu_name = cpu.processor_version;
                        let cpu_cores = cpu.core_count;
                        let cpu_threads = cpu.thread_count;
                        let cpu_freq = cpu.max_speed;
                        return Ok(format!(
                            "{} {} Cores {} Threads @ {}MHz",
                            cpu_name.trim(),
                            cpu_cores.unwrap_or_default(),
                            cpu_threads.unwrap_or_default(),
                            cpu_freq
                        ));
                    }
                    _ => continue,
                },
                Err(_) => return Ok(classic_get_cpu()),
            }
        }

        Ok("Read failed? Please report bug.".to_string())
    } else {
        Ok(classic_get_cpu())
    }
}

fn classic_get_cpu() -> String {
    let s = sysinfo::System::new_all();
    match s.cpus().first() {
        Some(cpu) => {
            let cpu_model = cpu.brand();
            let cpu_threads = s.cpus().len();
            let cpu_speed = cpu.frequency();
            format!("{cpu_model} {cpu_threads} Threads @ {cpu_speed}Mhz")
        }
        None => "Unknown CPU".to_string(),
    }
}
