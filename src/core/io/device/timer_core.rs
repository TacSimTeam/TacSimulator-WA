use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;

#[derive(Clone)]
pub struct TimerCore {
    pub count: u32,
    pub cycle: u32,
    pub timer_num: u8,
    pub match_flag: bool,
    pub intr_flag: bool,
    pub pause_flag: bool,
    pub intr_sig: IntrController,
}

impl TimerCore {
    pub fn new(timer_num: u8, intr_sig: IntrController) -> Self {
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

    pub fn get_count(&self) -> u32 {
        self.count
    }

    pub fn set_cycle(&mut self, cycle: u32) {
        self.cycle = cycle;
    }

    pub fn set_intr_flag(&mut self, flag: bool) {
        self.intr_flag = flag;
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

            if self.intr_flag {
                // TODO タイマーの番号に応じて割り込みの種類を変える. Fromトレイトを実装できればベストだけど一旦ifで処理
                if self.timer_num == 0 {
                    self.intr_sig.interrupt(Interrupt::TIMER0)
                } else {
                    self.intr_sig.interrupt(Interrupt::TIMER1);
                }
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
