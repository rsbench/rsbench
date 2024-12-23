#[allow(dead_code)]
pub fn get_swap() -> String {
    let proc_swaps = std::fs::read_to_string("/proc/swaps");
    match proc_swaps {
        Ok(swaps) => {
            let mut total_size_kb = 0;
            for line in swaps.lines().skip(1) {
                let parts: Vec<&str> = line.split_whitespace().collect();
                // 0: name, 1: type, 2: size, 3: used, 4: priority
                if let Some(swap_size) = parts.get(2) {
                    match swap_size.parse::<u64>() {
                        Ok(n) => {
                            total_size_kb += n;
                        }
                        Err(_) => continue,
                    }
                };
            }
            if total_size_kb < 1_048_575 {
                format!("{:.2} MB", total_size_kb as f64 / 1_024.0)
            } else {
                format!("{:.2} GB", total_size_kb as f64 / 1_048_575.0)
            }
        }
        Err(_) => "Error - Please report to issues.".to_string(),
    }
}
