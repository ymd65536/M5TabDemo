use std::{thread, time::Duration};

fn main() {
    println!("Hello from Rust on M5Stack Tab5 / ESP32-P4");

    loop {
        thread::sleep(Duration::from_secs(5));
    }
}
