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

#[cfg(target_os = "windows")]
pub fn platform_name() -> &'static str {
    "windows"
}

#[cfg(target_os = "macos")]
pub fn platform_name() -> &'static str {
    "macos"
}

#[cfg(target_os = "linux")]
pub fn platform_name() -> &'static str {
    "linux"
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn platform_name() -> &'static str {
    "unknown"
}

pub fn is_windows() -> bool {
    cfg!(target_os = "windows")
}

pub fn is_macos() -> bool {
    cfg!(target_os = "macos")
}

pub fn is_linux() -> bool {
    cfg!(target_os = "linux")
}
