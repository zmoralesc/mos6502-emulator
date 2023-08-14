use super::*;
use std::cmp::Ordering;

impl<T: Bus + Send + Sync> MOS6502<T> {
    pub(super) fn cmp(&mut self, address_mode: AddressingMode) {
        let operand: u8 = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w),
            _ => panic!("Invalid addressing mode."),
        };

        self.increment_cycles(1);
        let result = self.accumulator.wrapping_sub(operand);
        self.flag_toggle(FLAG_NEGATIVE, result & NEGATIVE_BIT_MASK != 0);

        match self.accumulator.cmp(&operand) {
            Ordering::Less => {
                self.flag_toggle(FLAG_ZERO, false);
                self.flag_toggle(FLAG_CARRY, false);
            }
            Ordering::Equal => {
                self.flag_toggle(FLAG_ZERO, true);
                self.flag_toggle(FLAG_CARRY, true);
            }
            Ordering::Greater => {
                self.flag_toggle(FLAG_ZERO, false);
                self.flag_toggle(FLAG_CARRY, true);
            }
        }
    }

    pub(super) fn cpx(&mut self, address_mode: AddressingMode) {
        let operand: u8 = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w),
            _ => panic!("Invalid addressing mode."),
        };

        self.increment_cycles(1);
        let result = self.x_register.wrapping_sub(operand);
        self.flag_toggle(FLAG_NEGATIVE, result & NEGATIVE_BIT_MASK != 0);

        match self.x_register.cmp(&operand) {
            Ordering::Less => {
                self.flag_toggle(FLAG_ZERO, false);
                self.flag_toggle(FLAG_CARRY, false);
            }
            Ordering::Equal => {
                self.flag_toggle(FLAG_ZERO, true);
                self.flag_toggle(FLAG_CARRY, true);
            }
            Ordering::Greater => {
                self.flag_toggle(FLAG_ZERO, false);
                self.flag_toggle(FLAG_CARRY, true);
            }
        }
    }

    pub(super) fn cpy(&mut self, address_mode: AddressingMode) {
        let operand: u8 = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w),
            _ => panic!("Invalid addressing mode."),
        };

        self.increment_cycles(1);
        let result = self.y_register.wrapping_sub(operand);
        self.flag_toggle(FLAG_NEGATIVE, result & NEGATIVE_BIT_MASK != 0);

        match self.y_register.cmp(&operand) {
            Ordering::Less => {
                self.flag_toggle(FLAG_ZERO, false);
                self.flag_toggle(FLAG_CARRY, false);
            }
            Ordering::Equal => {
                self.flag_toggle(FLAG_ZERO, true);
                self.flag_toggle(FLAG_CARRY, true);
            }
            Ordering::Greater => {
                self.flag_toggle(FLAG_ZERO, false);
                self.flag_toggle(FLAG_CARRY, true);
            }
        }
    }
}
