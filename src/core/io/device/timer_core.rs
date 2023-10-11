use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use std::sync::{Arc, Mutex};

#[derive(Clone, PartialEq)]
pub enum TimerNum {
    TIMER0,
    TIMER1,
}

#[derive(Clone)]
pub struct TimerCore {
    pub count: u16,
    pub cycle: u16,
    pub timer_num: TimerNum,
    pub match_flag: bool,
    pub intr_flag: bool,
    pub pause_flag: bool,
    pub intr_sig: Arc<Mutex<IntrController>>,
}

impl TimerCore {
    pub fn new(timer_num: TimerNum, intr_sig: Arc<Mutex<IntrController>>) -> Self {
        Self {
            count: 0,
            cycle: 0,
            timer_num,
            match_flag: false,
            intr_flag: false,
            pause_flag: false,
            intr_sig,
        }
    }

    pub fn get_count(&self) -> u16 {
        self.count
    }

    pub fn set_cycle(&mut self, cycle: u16) {
        self.cycle = cycle;
    }

    pub fn is_matched(&self) -> bool {
        self.match_flag
    }

    pub fn routine(&mut self) {
        if self.pause_flag {
            return;
        }

        if self.count == self.cycle {
            self.count = 0;
            self.match_flag = true;
            let mut clone = self.intr_sig.lock().unwrap();
            if self.timer_num == TimerNum::TIMER0 {
                clone.interrupt(Interrupt::TIMER0);
            } else {
                clone.interrupt(Interrupt::TIMER1);
            }
        } else {
            self.count += 1;
        }
    }

    pub fn pause(&mut self) {
        self.pause_flag = true;
    }

    pub fn reset(&mut self) {
        self.count = 0;
        self.count = 0;
        self.match_flag = false;
        self.intr_flag = false;
        self.pause_flag = false;
    }
}
