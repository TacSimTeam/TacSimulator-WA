use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn set_interval<F>(
    interval: Duration,
    running: &Arc<Mutex<bool>>,
    mut callback: F,
) -> thread::JoinHandle<()>
where
    F: FnMut() + Send + 'static,
{
    let running_clone = Arc::clone(running);

    thread::spawn(move || {
        while *running_clone.lock().unwrap() {
            callback();
            thread::sleep(interval);
        }
    })
}

pub fn clear_interval(handle: thread::JoinHandle<()>, running: &Arc<Mutex<bool>>) {
    *running.lock().unwrap() = false;
    handle.join().unwrap();
}
