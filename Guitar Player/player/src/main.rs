use log::{info, warn, LevelFilter};
use simple_logging::{log_to_file};

const LOG_FILE: &str = "logs.log";

fn test_0(msg: &str) {
    info!("{}", msg);
}

fn test_1(msg: &str) {
    warn!("{}", msg);
}

fn main() {
    let _ = log_to_file(LOG_FILE, LevelFilter::Info);
    test_0("Info message");
    test_1("Warning message");
}
