use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn cmp(&mut self, address_mode: AddressingMode) {
        let operand: u8 = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.bus.read(w),
            _ => panic!("Invalid addressing mode."),
        };

        self.increment_cycles(1);
        let result = self.accumulator.wrapping_sub(operand);

        if self.accumulator < operand {
            self.flag_toggle(FLAG_ZERO, false);
            self.flag_toggle(FLAG_CARRY, false);
        } else if self.accumulator == operand {
            self.flag_toggle(FLAG_ZERO, true);
            self.flag_toggle(FLAG_CARRY, true);
        } else {
            self.flag_toggle(FLAG_ZERO, false);
            self.flag_toggle(FLAG_CARRY, true);
        }
        self.flag_toggle(FLAG_NEGATIVE, result & NEGATIVE_BIT_MASK != 0);
    }

    pub(super) fn cpx(&mut self, address_mode: AddressingMode) {
        let operand: u8 = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.bus.read(w),
            _ => panic!("Invalid addressing mode."),
        };

        self.increment_cycles(1);
        let result = self.x_register.wrapping_sub(operand);

        if self.x_register < operand {
            self.flag_toggle(FLAG_ZERO, false);
            self.flag_toggle(FLAG_CARRY, false);
        } else if self.x_register == operand {
            self.flag_toggle(FLAG_ZERO, true);
            self.flag_toggle(FLAG_CARRY, true);
        } else {
            self.flag_toggle(FLAG_ZERO, false);
            self.flag_toggle(FLAG_CARRY, true);
        }
        self.flag_toggle(FLAG_NEGATIVE, result & NEGATIVE_BIT_MASK != 0);
    }

    pub(super) fn cpy(&mut self, address_mode: AddressingMode) {
        let operand: u8 = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.bus.read(w),
            _ => panic!("Invalid addressing mode."),
        };

        self.increment_cycles(1);
        let result = self.y_register.wrapping_sub(operand);

        if self.y_register < operand {
            self.flag_toggle(FLAG_ZERO, false);
            self.flag_toggle(FLAG_CARRY, false);
        } else if self.y_register == operand {
            self.flag_toggle(FLAG_ZERO, true);
            self.flag_toggle(FLAG_CARRY, true);
        } else {
            self.flag_toggle(FLAG_ZERO, false);
            self.flag_toggle(FLAG_CARRY, true);
        }
        self.flag_toggle(FLAG_NEGATIVE, result & NEGATIVE_BIT_MASK != 0);
    }
}
