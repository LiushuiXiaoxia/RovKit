use log::{debug, error, info, trace, warn, LevelFilter};
use std::sync::Once;

static INIT: Once = Once::new();

/// 初始化日志系统，只初始化一次
pub fn init_log(level: LevelFilter) {
    INIT.call_once(|| {
        env_logger::builder()
            .format(|buf, record| {
                use std::io::Write;
                writeln!(
                    buf,
                    "{} [{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f"),
                    record.level(),
                    record.args()
                )
            })
            .filter_level(level)
            .init();
    });
}

/// 快速记录 info 级别日志
pub fn log_info(msg: &str) {
    info!("{}", msg);
}

/// 快速记录 warn 级别日志
pub fn log_warn(msg: &str) {
    warn!("{}", msg);
}

/// 快速记录 error 级别日志
pub fn log_error(msg: &str) {
    error!("{}", msg);
}

/// 快速记录 debug 级别日志
pub fn log_debug(msg: &str) {
    debug!("{}", msg);
}

/// 快速记录 trace 级别日志
pub fn log_trace(msg: &str) {
    trace!("{}", msg);
}

/// 快速记录 info 级别日志
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
       log::info!("{}", format!($($arg)*));
    };
}

/// 快速记录 warn 级别日志
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
       log:: warn!("{}", format!($($arg)*));
    };
}

/// 快速记录 error 级别日志
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!("{}", format!($($arg)*));
    };
}

/// 快速记录 debug 级别日志
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!("{}", format!($($arg)*));
    };
}

/// 快速记录 trace 级别日志
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        log::trace!("{}", format!($($arg)*));
    };
}
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_log_macros() {
//         init_log(LevelFilter::Info);
//         log_info("Info message");
//         log_warn("Warn message");
//         log_error("Error message");
//         log_debug("Debug message");
//         log_trace("Trace message");
//     }
// }
