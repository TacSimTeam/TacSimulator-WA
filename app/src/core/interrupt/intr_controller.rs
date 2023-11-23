use crate::core::interrupt::interrupt::Interrupt;

#[derive(Clone, Debug, PartialEq)]
pub struct IntrController {
    intr_flags: Vec<bool>,
}

impl IntrController {
    pub fn new() -> Self {
        Self {
            intr_flags: vec![false; 16],
        }
    }

    pub fn reset(&mut self) {
        self.intr_flags.fill(false);
    }

    pub fn interrupt(&mut self, intr_num: Interrupt) {
        self.intr_flags[intr_num as usize] = true;
    }

    pub fn check_intr_num(&mut self) -> Option<u8> {
        for i in 10..=15 {
            if self.intr_flags[i] {
                self.intr_flags[i] = false;
                return Some(i as u8);
            }
        }

        for i in 0..=9 {
            if self.intr_flags[i] {
                self.intr_flags[i] = false;
                return Some(i as u8);
            }
        }
        None
    }

    pub fn is_exception_occurred(&self) -> bool {
        for i in 10..=15 {
            if self.intr_flags[i] {
                return true;
            }
        }
        false
    }
}
