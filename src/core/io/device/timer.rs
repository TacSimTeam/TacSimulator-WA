use crate::core::interrupt::intr_controller::IntrController;
use crate::core::io::device::timer_core::{TimerCore, TimerNum};
use crate::util::interval::{clear_interval, set_interval};
use std::ops::{Deref, Index};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

pub struct Timer {
    interval_id: Option<JoinHandle<()>>,
    pub timer0: Arc<Mutex<TimerCore>>,
    pub timer1: Arc<Mutex<TimerCore>>,
    is_timer0_running: Arc<Mutex<bool>>,
    is_timer1_running: Arc<Mutex<bool>>,
}

impl Timer {
    pub fn new(intr_sig: Arc<Mutex<IntrController>>) -> Self {
        Self {
            interval_id: None,
            timer0: Arc::new(Mutex::new(TimerCore::new(
                TimerNum::TIMER0,
                Arc::clone(&intr_sig),
            ))),
            timer1: Arc::new(Mutex::new(TimerCore::new(TimerNum::TIMER1, intr_sig))),
            is_timer0_running: Arc::new(Mutex::new(false)),
            is_timer1_running: Arc::new(Mutex::new(false)),
        }
    }

    pub fn start_timer(&mut self, timer_num: TimerNum) {
        let timer_clone = match timer_num {
            TimerNum::TIMER0 => (Arc::clone(&self.timer0), &self.is_timer0_running),
            TimerNum::TIMER1 => (Arc::clone(&self.timer0), &self.is_timer1_running),
        };
        let interval = set_interval(Duration::from_millis(1), timer_clone.1, move || {
            let mut timer = timer_clone.0.lock().unwrap();
            timer.routine();
        });
        self.interval_id = Some(interval);
    }

    pub fn clear_timer(&mut self, timer_num: TimerNum) {
        if self.interval_id.is_some() {
            let is_running = match timer_num {
                TimerNum::TIMER0 => &self.is_timer0_running,
                TimerNum::TIMER1 => &self.is_timer1_running,
            };
            clear_interval(self.interval_id.take().unwrap(), is_running);
            self.interval_id = None;
        }
    }

    pub fn pause_timer(&mut self, timer_num: TimerNum) {
        let timer = match timer_num {
            TimerNum::TIMER0 => Arc::clone(&self.timer0),
            TimerNum::TIMER1 => Arc::clone(&self.timer1),
        };
        timer.lock().unwrap().pause_flag = true;
    }

    pub fn restart_timer(&mut self, timer_num: TimerNum) {
        let timer = match timer_num {
            TimerNum::TIMER0 => Arc::clone(&self.timer0),
            TimerNum::TIMER1 => Arc::clone(&self.timer1),
        };
        timer.lock().unwrap().pause_flag = false;
    }

    pub fn get_counter(&self, timer_num: TimerNum) -> u16 {
        match timer_num {
            TimerNum::TIMER0 => self.timer0.lock().unwrap().get_count(),
            TimerNum::TIMER1 => self.timer1.lock().unwrap().get_count(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::core::interrupt::intr_controller::IntrController;
    use crate::core::io::device::timer::Timer;
    use crate::core::io::device::timer_core::TimerNum;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn timer_test() {
        let intr_controller = IntrController::new();
        let mut timer = Timer::new(Arc::new(Mutex::new(intr_controller)));
        timer.timer0.lock().unwrap().set_cycle(100);
        timer.start_timer(TimerNum::TIMER0);
        thread::sleep(Duration::from_secs(2));
        timer.clear_timer(TimerNum::TIMER0);
        println!("count: {}", timer.timer0.lock().unwrap().get_count());
    }
}
