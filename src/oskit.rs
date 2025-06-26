use std::path::Path;
use sysinfo::{Disks, System};

pub struct OsInfo {
    pub os_name: Option<String>,
    pub kernel_version: Option<String>,
    pub cpu_brand: String,
    pub cpu_cores: usize,
    pub total_memory: u64,
    pub used_memory: u64,
    pub disks: Vec<(String, u64)>, // (mount point, total space)
}

pub struct DiskUsage {
    pub mount_point: String,
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

pub struct MemoryInfo {
    pub total: u64,
    pub used: u64,
    pub free: u64,
}

pub fn get_os_info() -> OsInfo {
    let mut sys = System::new_all();
    sys.refresh_all();

    let disks = Disks::new()
        .iter()
        .map(|d| {
            (
                d.mount_point().to_string_lossy().to_string(),
                d.total_space(),
            )
        })
        .collect();

    OsInfo {
        os_name: System::name(),
        kernel_version: System::kernel_version(),
        cpu_brand: sys.global_cpu_info().brand().to_string(),
        cpu_cores: sys.cpus().len(),
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        disks,
    }
}

/// 获取指定路径所在磁盘的空间信息
pub fn get_disk_usage<P: AsRef<Path>>(path: P) -> Option<DiskUsage> {
    let mut disks = Disks::new();
    disks.refresh_list();
    for disk in disks.list() {
        if path.as_ref().starts_with(disk.mount_point()) {
            return Some(DiskUsage {
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total: disk.total_space(),
                available: disk.available_space(),
                used: disk.total_space() - disk.available_space(),
            });
        }
    }
    None
}

/// 获取系统内存信息
pub fn get_memory_info() -> MemoryInfo {
    let mut sys = System::new();
    sys.refresh_memory();
    let total = sys.total_memory();
    let used = sys.used_memory();
    let free = total - used;
    MemoryInfo { total, used, free }
}

/// 获取 CPU 核心数
pub fn get_cpu_cores() -> usize {
    let mut sys = System::new();
    sys.refresh_cpu();
    sys.cpus().len()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
