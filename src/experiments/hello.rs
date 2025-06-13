use std::time::Duration;
use std::thread;

pub fn run() {
    thread::sleep(Duration::from_millis(500));
    println!("[SYSTEM] Starting Hello");
    thread::sleep(Duration::from_secs(1));
    println!("Hello World!");
    thread::sleep(Duration::from_secs(1));
    println!("[SYSTEM] Hello Finished Execution");
    thread::sleep(Duration::from_millis(500));
}
