use crate::core::cpu::consts::opcode::OPCode;
use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Clone, Debug)]
pub struct Alu {
    intr_sig: Rc<RefCell<IntrController>>,
}

impl Alu {
    pub fn new(intr_sig: Rc<RefCell<IntrController>>) -> Self {
        Self { intr_sig }
    }

    pub fn calc(&mut self, op: OPCode, left: u16, right: u16) -> u32 {
        match op {
            OPCode::ADD => (left as i32 + right as i32) as u32,
            OPCode::SUB => (left as i32 - right as i32) as u32,
            OPCode::CMP => (left as i32 - right as i32) as u32,
            OPCode::AND => left as u32 & right as u32,
            OPCode::OR => left as u32 | right as u32,
            OPCode::XOR => left as u32 ^ right as u32,
            OPCode::ADDS => (left as i32 + (right as i32) * 2) as u32,
            OPCode::MUL => left as u32 * right as u32,
            OPCode::DIV => self.div(left, right),
            OPCode::MOD => self.modulo(left, right),
            OPCode::SHLA => self.shift_left(left, right),
            OPCode::SHLL => self.shift_left(left, right),
            OPCode::SHRA => self.shift_right_arithmetic(left, right),
            OPCode::SHRL => self.shift_right_logical(left, right),
            _ => 0u32,
        }
    }

    fn div(&mut self, dividend: u16, divisor: u16) -> u32 {
        let dividend = dividend as u32;
        let divisor = divisor as u32;
        if divisor == 0 {
            self.intr_sig
                .borrow_mut()
                .interrupt(Interrupt::EXCP_ZERO_DIV);
            return 0;
        }
        dividend / divisor
    }

    fn modulo(&mut self, dividend: u16, divisor: u16) -> u32 {
        let dividend = dividend as u32;
        let divisor = divisor as u32;
        if divisor == 0 {
            self.intr_sig
                .borrow_mut()
                .interrupt(Interrupt::EXCP_ZERO_DIV);
            return 0;
        }
        dividend % divisor
    }

    fn shift_left(&mut self, operand: u16, bit: u16) -> u32 {
        let operand = operand as u32;
        let bit = bit as u32;
        operand << (bit & 0x0f)
    }

    fn shift_right_arithmetic(&mut self, operand: u16, bit: u16) -> u32 {
        let operand = operand as u32;
        let bit = bit as u32;
        if (operand & 0x8000) != 0 {
            return ((operand | !0xffff) as i32 >> (bit & 0x0f)) as u32;
        }
        return operand >> (bit & 0x0f);
    }

    fn shift_right_logical(&mut self, operand: u16, bit: u16) -> u32 {
        let operand = operand as u32;
        let bit = bit as u32;
        operand >> (bit & 0x0f)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::alu::Alu;
    use crate::core::cpu::consts::opcode::OPCode;
    use crate::core::interrupt::interrupt::Interrupt;
    use crate::core::interrupt::intr_controller::IntrController;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_calc() {
        let mut intr_sig = Rc::new(RefCell::new(IntrController::new()));
        let mut alu = Alu::new(Rc::clone(&intr_sig));

        assert_eq!(alu.calc(OPCode::ADD, 0x0001, 0x0001), 0x0002);
        assert_eq!(alu.calc(OPCode::ADD, 0x0001, 0xffff) as u16, 0x0000);
        assert_eq!(alu.calc(OPCode::ADD, 0xffff, 0xffff) as u16, 0xfffe);

        assert_eq!(alu.calc(OPCode::SUB, 0x0001, 0x0001), 0x0000);
        assert_eq!(alu.calc(OPCode::SUB, 0x0002, 0x0001), 0x0001);
        assert_eq!(alu.calc(OPCode::SUB, 0x0001, 0x0002) as u16, 0xffff);
        assert_eq!(alu.calc(OPCode::SUB, 0x0001, 0xffff) as u16, 0x0002);
        assert_eq!(alu.calc(OPCode::SUB, 0xfffe, 0xffff) as u16, 0xffff);

        assert_eq!(alu.calc(OPCode::CMP, 0x0001, 0x0001), 0x0000);
        assert_eq!(alu.calc(OPCode::CMP, 0x0002, 0x0001), 0x0001);
        assert_eq!(alu.calc(OPCode::CMP, 0x0001, 0x0002) as u16, 0xffff);
        assert_eq!(alu.calc(OPCode::CMP, 0x0001, 0xffff) as u16, 0x0002);
        assert_eq!(alu.calc(OPCode::CMP, 0xfffe, 0xffff) as u16, 0xffff);

        assert_eq!(alu.calc(OPCode::AND, 0x00ff, 0x1234), 0x0034);
        assert_eq!(alu.calc(OPCode::AND, 0x0000, 0xffff), 0);

        assert_eq!(alu.calc(OPCode::OR, 0x00ff, 0x1234), 0x12ff);
        assert_eq!(alu.calc(OPCode::OR, 0x0000, 0xffff), 0xffff);

        assert_eq!(alu.calc(OPCode::XOR, 0x5555, 0xffff), 0xaaaa);
        assert_eq!(alu.calc(OPCode::XOR, 0x5555, 0x0000), 0x5555);

        assert_eq!(alu.calc(OPCode::ADDS, 0x0002, 0x0001), 0x0004);
        assert_eq!(alu.calc(OPCode::ADDS, 0x0002, 0xffff) as u16, 0);

        assert_eq!(alu.calc(OPCode::MUL, 0x0003, 0x0004), 0x000c);
        assert_eq!(alu.calc(OPCode::MUL, 0xffff, 0x0001), 0xffff);

        assert_eq!(alu.calc(OPCode::DIV, 0x0008, 0x0002), 0x0004);
        assert_eq!(alu.calc(OPCode::DIV, 0x0009, 0x0002), 0x0004);
        assert_eq!(alu.calc(OPCode::DIV, 0x0001, 0x0002), 0x0000);

        assert_eq!(alu.calc(OPCode::DIV, 0x0001, 0x0000), 0);
        assert_eq!(
            intr_sig.borrow_mut().check_intr_num(),
            Some(Interrupt::EXCP_ZERO_DIV as u8)
        );

        assert_eq!(alu.calc(OPCode::MOD, 0x0008, 0x0002), 0x0000);
        assert_eq!(alu.calc(OPCode::MOD, 0x0009, 0x0002), 0x0001);
        assert_eq!(alu.calc(OPCode::MOD, 0x00ff, 0xffff), 0x00ff);

        assert_eq!(alu.calc(OPCode::MOD, 0x0001, 0x0000), 0);
        assert_eq!(
            intr_sig.borrow_mut().check_intr_num(),
            Some(Interrupt::EXCP_ZERO_DIV as u8)
        );

        assert_eq!(alu.calc(OPCode::SHLA, 0x0055, 0x0001) as u16, 0x00aa);
        assert_eq!(alu.calc(OPCode::SHLA, 0xff00, 0x0004) as u16, 0xf000);

        assert_eq!(alu.calc(OPCode::SHLL, 0x0055, 0x0001), 0x00aa);
        assert_eq!(alu.calc(OPCode::SHLL, 0xff00, 0x0004) as u16, 0xf000);

        assert_eq!(alu.calc(OPCode::SHRA, 0x00aa, 0x0001), 0x0055);
        assert_eq!(alu.calc(OPCode::SHRA, 0x8000, 0x0008) as u16, 0xff80);

        assert_eq!(alu.calc(OPCode::SHRL, 0x00aa, 0x0001), 0x0055);
        assert_eq!(alu.calc(OPCode::SHRL, 0x8000, 0x0008), 0x0080);
    }
}
