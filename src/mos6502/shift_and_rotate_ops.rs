use std::ops::{Shl, Shr};

use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn asl(&mut self, address_mode: AddressingMode) {
        match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(_) => {
                self.flag_toggle(FLAG_CARRY, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.accumulator <<= 1;
                self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
            }
            OpcodeOperand::Address(w) => {
                let value = self.bus.read(w);
                self.flag_toggle(FLAG_CARRY, value & 0b00000001 != 0);
                let new_value = value.shl(1);
                self.flag_toggle(FLAG_NEGATIVE, false);
                self.flag_toggle(FLAG_ZERO, new_value == 0);
                self.bus.write(w, new_value);
            }
            _ => panic!("Invalid addressing mode for ASL"),
        };
        self.increment_program_counter(1);
    }

    pub(super) fn lsr(&mut self, address_mode: AddressingMode) {
        match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(_) => self.accumulator >>= 1,
            OpcodeOperand::Address(w) => self.bus.write(w, self.bus.read(w).shr(1)),
            _ => panic!("Invalid addressing mode for LSR"),
        };
    }

    pub(super) fn rol(&mut self, address_mode: AddressingMode) {
        todo!()
    }

    pub(super) fn ror(&mut self, address_mode: AddressingMode) {
        todo!()
    }
}