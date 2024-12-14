#[allow(dead_code)]
pub fn get_virt() -> String {
    // On linux, we can use systemd-detect-virt if systemd is available
    #[cfg(target_os = "linux")]
    {
        // Only implemented for systemd systems for now, to add later
        std::process::Command::new("systemd-detect-virt")
            .output()
            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
            .unwrap_or_else(|_| "unknown".to_string())
    }

    #[cfg(not(target_os = "linux"))]
    {
        "unimplemented".to_string()
    }
}
