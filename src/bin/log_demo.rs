use log::LevelFilter;
use rovkit::*;

#[allow(dead_code)]
fn main() {
    logkit::init_log(LevelFilter::Debug);
    logkit::log_trace("Debugging mode enabled");
    logkit::log_debug("Debugging mode enabled");
    logkit::log_info("Starting application");
    logkit::log_warn("Starting application");
    logkit::log_error("Something went wrong ~~~");
}
