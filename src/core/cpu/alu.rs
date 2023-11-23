use crate::core::cpu::consts::opcode::OPCode;
use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

#[derive(PartialEq, Clone)]
pub struct Alu {
    intr_sig: Rc<RefCell<IntrController>>,
}

impl Alu {
    pub fn new(intr_sig: Rc<RefCell<IntrController>>) -> Self {
        Self { intr_sig }
    }

    pub fn calc(&mut self, op: OPCode, left: i16, right: i16) -> i32 {
        let left = left as i32;
        let right = right as i32;
        match op {
            OPCode::ADD => left + right,
            OPCode::SUB => left - right,
            OPCode::CMP => left - right,
            OPCode::AND => left & right,
            OPCode::OR => left | right,
            OPCode::XOR => left ^ right,
            OPCode::ADDS => left + right * 2,
            OPCode::MUL => left * right,
            OPCode::DIV => self.div(left, right),
            OPCode::MOD => self.modulo(left, right),
            OPCode::SHLA => self.shift_left(left, right),
            OPCode::SHLL => self.shift_left(left, right),
            OPCode::SHRA => self.shift_right_arithmetic(left, right),
            OPCode::SHRL => self.shift_right_logical(left, right),
            _ => 0i32,
        }
    }

    fn div(&mut self, dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            self.intr_sig
                .borrow_mut()
                .interrupt(Interrupt::EXCP_ZERO_DIV);
            return 0;
        }
        dividend / divisor
    }

    fn modulo(&mut self, dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 {
            self.intr_sig
                .borrow_mut()
                .interrupt(Interrupt::EXCP_ZERO_DIV);
            return 0;
        }
        dividend % divisor
    }

    fn shift_left(&mut self, operand: i32, bit: i32) -> i32 {
        operand << (bit & 0x0f)
    }

    fn shift_right_arithmetic(&mut self, operand: i32, bit: i32) -> i32 {
        if (operand as u32 & 0x8000) != 0 {
            return (operand as u32 | !0xffff) as i32 >> (bit & 0x0f);
        }
        return operand >> (bit & 0x0f);
    }

    fn shift_right_logical(&mut self, operand: i32, bit: i32) -> i32 {
        operand >> (bit & 0x0f)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::alu::Alu;
    use crate::core::cpu::consts::opcode::OPCode;
    use crate::core::interrupt::intr_controller::IntrController;
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_calc() {
        let intr_sig = IntrController::new();
        let mut alu = Alu::new(Rc::new(RefCell::new(intr_sig)));

        let left = 1;
        let right = 2;
        assert_eq!(alu.calc(OPCode::ADD, left, right), 3);
        assert_eq!(alu.calc(OPCode::SUB, left, right), -1);
        assert_eq!(alu.calc(OPCode::SHRA, -1, 2), -1);
    }
}
