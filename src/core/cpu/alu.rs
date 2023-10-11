use crate::core::cpu::consts::opcode::OPCode;
use crate::core::interrupt::interrupt::Interrupt;
use crate::core::interrupt::intr_controller::IntrController;

pub struct Alu {
    intr_sig: IntrController,
}

impl Alu {
    pub fn new(intr_sig: IntrController) -> Self {
        Self { intr_sig }
    }

    pub fn calc(&mut self, op: OPCode, left: i16, right: i16) -> i16 {
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
            _ => 0i16,
        }
    }

    fn div(&mut self, dividend: i16, divisor: i16) -> i16 {
        if divisor == 0 {
            self.intr_sig.interrupt(Interrupt::EXCP_ZERO_DIV);
            return 0;
        }
        dividend / divisor
    }

    fn modulo(&mut self, dividend: i16, divisor: i16) -> i16 {
        if divisor == 0 {
            self.intr_sig.interrupt(Interrupt::EXCP_ZERO_DIV);
            return 0;
        }
        dividend % divisor
    }

    fn shift_left(&mut self, operand: i16, bit: i16) -> i16 {
        operand << (bit & 0x0f)
    }

    fn shift_right_arithmetic(&mut self, operand: i16, bit: i16) -> i16 {
        if (operand as u16 & 0x8000) != 0 {
            return (operand as u16 | !0xffff) as i16 >> (bit & 0x0f);
        }
        return operand >> (bit & 0x0f);
    }

    fn shift_right_logical(&mut self, operand: i16, bit: i16) -> i16 {
        operand >> (bit & 0x0f)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::cpu::alu::Alu;
    use crate::core::cpu::consts::opcode::OPCode;
    use crate::core::interrupt::intr_controller::IntrController;

    #[test]
    fn test_calc() {
        let intr_sig = IntrController::new();
        let mut alu = Alu::new(intr_sig);

        let left = 1;
        let right = 2;
        assert_eq!(alu.calc(OPCode::ADD, left, right), 3);
        assert_eq!(alu.calc(OPCode::SUB, left, right), -1);
        assert_eq!(alu.calc(OPCode::SHRA, -1, 2), -1);
    }
}
