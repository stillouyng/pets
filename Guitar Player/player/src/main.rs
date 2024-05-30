use log::{info, warn, LevelFilter};
use simple_logging::{log_to_file};
use std::{thread, time};


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
    let sleep_duration = time::Duration::from_millis(2000);
    let now = time::Instant::now();
    println!("Sleep start: {:?}", now);
    thread::sleep(sleep_duration);
    let after = time::Instant::now();
    println!("Sleep end: {:?}", after);
    test_1("Warning message");
}
