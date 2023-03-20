use std::thread;
use std::time::Duration;

pub fn sleep(seconds: f32) {
    let millis: u64 = (seconds * 1000.0) as u64;
    let duration: Duration = Duration::from_millis(millis);
    thread::sleep(duration);
}
