use crate::core::interrupt::intr_controller::IntrController;
use crate::core::io::device::timer_core::{TimerCore, TimerNum};
use crate::util::interval::{clear_interval, set_interval};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub struct Timer {
    interval_id_timer0: Option<i32>,
    interval_id_timer1: Option<i32>,
    pub timer0: Rc<RefCell<TimerCore>>,
    pub timer1: Rc<RefCell<TimerCore>>,
}

impl Timer {
    pub fn new(intr_sig: Rc<RefCell<IntrController>>) -> Self {
        Self {
            interval_id_timer0: None,
            interval_id_timer1: None,
            timer0: Rc::new(RefCell::new(TimerCore::new(
                TimerNum::TIMER0,
                Rc::clone(&intr_sig),
            ))),
            timer1: Rc::new(RefCell::new(TimerCore::new(
                TimerNum::TIMER1,
                Rc::clone(&intr_sig),
            ))),
        }
    }

    pub fn start_timer(&mut self, timer_num: TimerNum) {
        let timer_clone = match timer_num {
            TimerNum::TIMER0 => Rc::clone(&self.timer0),
            TimerNum::TIMER1 => Rc::clone(&self.timer1),
        };
        let interval = set_interval(timer_clone, 1);
        match timer_num {
            TimerNum::TIMER0 => {
                self.interval_id_timer0 = Some(interval);
            }
            TimerNum::TIMER1 => {
                self.interval_id_timer1 = Some(interval);
            }
        }
    }

    pub fn clear_timer(&mut self, timer_num: TimerNum) {
        let interval_id = match timer_num {
            TimerNum::TIMER0 => self.interval_id_timer0,
            TimerNum::TIMER1 => self.interval_id_timer1,
        };
        if interval_id.is_some() {
            clear_interval(interval_id.unwrap());
        }
        match timer_num {
            TimerNum::TIMER0 => {
                self.interval_id_timer0 = None;
            }
            TimerNum::TIMER1 => {
                self.interval_id_timer1 = None;
            }
        }
    }

    pub fn pause_timer(&mut self, timer_num: TimerNum) {
        match timer_num {
            TimerNum::TIMER0 => Rc::clone(&self.timer0),
            TimerNum::TIMER1 => Rc::clone(&self.timer1),
        }
        .borrow_mut()
        .pause_flag = true;
    }

    pub fn restart_timer(&mut self, timer_num: TimerNum) {
        match timer_num {
            TimerNum::TIMER0 => Rc::clone(&self.timer0),
            TimerNum::TIMER1 => Rc::clone(&self.timer1),
        }
        .borrow_mut()
        .pause_flag = false;
    }

    pub fn get_counter(&self, timer_num: TimerNum) -> u16 {
        match timer_num {
            TimerNum::TIMER0 => &self.timer0,
            TimerNum::TIMER1 => &self.timer1,
        }
        .borrow()
        .get_count()
    }

    pub fn stop_timer(&mut self, timer_num: TimerNum) {
        self.clear_timer(timer_num);
    }
}