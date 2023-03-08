use std::{thread, time};

pub fn sleep(seconds: f32) {
    let millis = (seconds * 1000.0) as u64;
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
