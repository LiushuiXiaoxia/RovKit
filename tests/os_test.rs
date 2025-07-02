#[cfg(test)]
mod tests {
    use rovkit::oskit::*;

    #[test]
    fn test_get_os_info() {
        let info = get_os_info();
        println!("OS: {:?}, Kernel: {:?}", info.os_name, info.kernel_version);
        println!("CPU: {}, Cores: {}", info.cpu_brand, info.cpu_cores);
        println!(
            "Memory: {} used / {} total",
            info.used_memory, info.total_memory
        );
        for (mount, space) in info.disks {
            println!("Disk: {} => {} bytes", mount, space);
        }
    }

    #[cfg(any(target_os = "macos", target_os = "linux"))]
    #[test]
    fn test_disk_usage() {
        let usage = get_disk_usage("/");
        assert!(usage.is_some());
        let u = usage.unwrap();
        println!(
            "Disk usage at {}: total={} avail={} used={}",
            u.mount_point, u.total, u.available, u.used
        );
    }

    #[cfg(any(target_os = "windows"))]
    #[test]
    fn test_disk_usage_window() {
        let usage = get_disk_usage("C:\\");
        assert!(usage.is_some());
        let u = usage.unwrap();
        println!(
            "Disk usage at {}: total={} avail={} used={}",
            u.mount_point, u.total, u.available, u.used
        );
    }

    #[test]
    fn test_memory_info() {
        let mem = get_memory_info();
        println!(
            "Memory: {} total, {} used, {} free",
            mem.total, mem.used, mem.free
        );
        assert!(mem.total >= mem.used);
    }

    #[test]
    fn test_cpu_cores() {
        let cores = get_cpu_cores();
        println!("CPU Cores: {}", cores);
        assert!(cores > 0);
    }

    #[test]
    fn test_os() {
        println!("platform = {}", platform_name());
        println!("is_macos = {}", is_macos());
        println!("is_linux = {}", is_linux());
        println!("is_windows = {}", is_windows());
    }
}
