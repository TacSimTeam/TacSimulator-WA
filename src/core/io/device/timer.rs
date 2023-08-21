use crate::core::interrupt::intr_controller::IntrController;
use crate::core::io::device::timer_core::TimerCore;
use crate::util::interval::{clear_interval, set_interval};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Timer {
    interval_id: Option<JoinHandle<()>>,
    timer: Arc<Mutex<TimerCore>>,
    is_running: Arc<Mutex<bool>>,
}

impl Timer {
    pub fn new(timer_num: u8, intr_sig: IntrController) -> Self {
        Self {
            interval_id: None,
            timer: Arc::new(Mutex::new(TimerCore::new(timer_num, intr_sig))),
            is_running: Arc::new(Mutex::new(true)),
        }
    }

    pub fn start(&mut self) {
        let timer_clone = Arc::clone(&self.timer);
        let interval = set_interval(Duration::from_millis(1), &self.is_running, move || {
            let mut timer = timer_clone.lock().unwrap();
            timer.routine();
        });
        self.interval_id = Some(interval);
    }

    pub fn clear(&mut self) {
        if self.interval_id.is_some() {
            clear_interval(self.interval_id.take().unwrap(), &self.is_running);
            self.interval_id = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::interrupt::intr_controller::IntrController;
    use crate::core::io::device::timer::Timer;
    use log::info;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn timer_test() {
        let mut timer = Timer::new(0, IntrController::new());
        timer.start();
        timer.timer.lock().unwrap().set_cycle(1000000);
        thread::sleep(Duration::from_secs(5));
        timer.clear();
        info!("count: {}", timer.timer.lock().unwrap().get_count());
    }
}
