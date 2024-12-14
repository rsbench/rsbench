use std::process::Command;

#[allow(dead_code)]
pub fn get_cpu_model() -> String {
    #[cfg(target_os = "linux")]
    {
        use std::{
            fs::File,
            io::{BufRead, BufReader},
        };
        if let Ok(file) = File::open("/proc/cpuinfo") {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(line) = line {
                    if line.starts_with("model name") {
                        if let Some(model) = line.split(':').nth(1) {
                            return model.trim().to_string();
                        }
                    }
                }
            }
        }
        "Unknown CPU".to_string()
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("sysctl")
            .arg("-n")
            .arg("machdep.cpu.brand_string")
            .output()
        {
            if let Ok(model) = String::from_utf8(output.stdout) {
                return model.trim().to_string();
            }
        }
        "Unknown CPU".to_string()
    }

    #[cfg(target_os = "windows")]
    {
        // Far from ideal, but I want to keep dependencies to a minimum
        if let Ok(output) = Command::new("wmic")
            .arg("cpu")
            .arg("get")
            .arg("name")
            .output()
        {
            if let Ok(model) = String::from_utf8(output.stdout) {
                let mut lines = model.lines();
                lines.next();
                if let Some(model) = lines.next() {
                    return model.trim().to_string();
                }
            }
        }
        "Unknown CPU".to_string()
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        "Unsupported OS, please open issue: https://github.com/rsbench/rsbench/issues".to_string()
    }
}
