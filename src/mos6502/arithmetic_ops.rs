use super::*;

impl<T: Bus> MOS6502<T> {
    // add to accumulator with carry
    pub(super) fn adc(&mut self, address_mode: AddressingMode) {
        let old_value = self.accumulator;
        let operand = self.resolve_operand(address_mode);
        let value = match operand {
            OpcodeOperand::Byte(b) => b,
            _ => {
                panic!("Invalid addressing mode for ADC");
            }
        };
        self.increment_cycles(1);

        let sign_bits_match: bool = ((self.accumulator ^ value) & SIGN_BIT_MASK) == 0;

        self.accumulator = self
            .accumulator
            .wrapping_add(value)
            .wrapping_add(self.flag_check(FLAG_CARRY) as u8);

        let overflow: bool = sign_bits_match && ((self.accumulator ^ value) & SIGN_BIT_MASK) != 0;

        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & SIGN_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_CARRY, self.accumulator < old_value);
        self.flag_toggle(FLAG_OVERFLOW, overflow);
    }

    pub(super) fn sbc(&mut self, address_mode: AddressingMode) {
        let old_value = self.accumulator;
        let operand = self.resolve_operand(address_mode);
        let value = match operand {
            OpcodeOperand::Byte(b) => b.wrapping_neg(),
            _ => {
                panic!("Invalid addressing mode for ADC");
            }
        };
        self.increment_cycles(1);

        let sign_bits_match: bool = ((self.accumulator ^ value) & SIGN_BIT_MASK) == 0;

        self.accumulator = self
            .accumulator
            .wrapping_add(value)
            .wrapping_add(self.flag_check(FLAG_CARRY) as u8);

        let overflow: bool = sign_bits_match && ((self.accumulator ^ value) & SIGN_BIT_MASK) != 0;

        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & SIGN_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_CARRY, self.accumulator < old_value);
        self.flag_toggle(FLAG_OVERFLOW, overflow);
    }
}
