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

    pub fn calc(&mut self, op: OPCode, left: u16, right: u16) -> u16 {
        match op {
            OPCode::ADD => return left + right,
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
            _ => 0u16,
        }
    }

    fn div(&mut self, dividend: u16, divisor: u16) -> u16 {
        if divisor == 0 {
            self.intr_sig.interrupt(Interrupt::EXCP_ZERO_DIV);
            return 0;
        }
        dividend / divisor
    }

    fn modulo(&mut self, dividend: u16, divisor: u16) -> u16 {
        if divisor == 0 {
            self.intr_sig.interrupt(Interrupt::EXCP_ZERO_DIV);
            return 0;
        }
        dividend % divisor
    }

    fn shift_left(&mut self, operand: u16, bit: u16) -> u16 {
        operand << (bit & 0x0f)
    }

    fn shift_right_arithmetic(&mut self, operand: u16, bit: u16) -> u16 {
        // TODO 論理シフトしてるから一回符号付きに直してシフトしてもう一回符号なしに戻す？
        if (operand & 0x8000) != 0 {
            return (operand | !0xffff) >> (bit & 0x0f);
        }
        return operand >> (bit & 0x0f);
    }

    fn shift_right_logical(&mut self, operand: u16, bit: u16) -> u16 {
        operand >> (bit & 0x0f)
    }
}
