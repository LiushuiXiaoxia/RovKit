#[cfg(test)]
mod tests {
    use log::LevelFilter;
    use rovkit::logkit::*;
    use rovkit::{log_debug, log_error, log_info, log_warn};

    #[test]
    fn test_log_macros() {
        init_log(LevelFilter::Info);
        log_info("Info message");
        log_warn("Warn message");
        log_error("Error message");
        log_debug("Debug message");
        log_trace("Trace message");
    }

    #[test]
    fn test_log() {
        init_log(LevelFilter::Debug);
        for i in 1..100 {
            log_debug!("Debugging mode enabled, i = {}", i);
            log_info!("Debugging mode enabled, i = {}", i);
            log_warn!("Debugging mode enabled, i = {}", i);
            log_error!("Debugging mode enabled, i = {}", i);
        }
        log_info!("Starting application");
        log_warn!("Starting application");
        log_error!("Something went wrong ~~~");
    }
}
