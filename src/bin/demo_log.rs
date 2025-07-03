use log::LevelFilter;
use rovkit::logkit::init_log;
use rovkit::{log_debug, log_error, log_info, log_warn};

#[allow(dead_code)]
fn main() {
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
