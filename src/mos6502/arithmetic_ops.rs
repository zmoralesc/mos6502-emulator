use super::*;

impl<T: Bus + Send + Sync> MOS6502<T> {
    // add to accumulator with carry
    pub(super) fn adc(&mut self, address_mode: AddressingMode) {
        let old_value = self.accumulator;
        let operand = self.resolve_operand(address_mode);
        let value = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.read_from_bus(addr),
            _ => {
                panic!("Invalid addressing mode for ADC");
            }
        };
        self.increment_cycles(1);

        let sign_bits_match: bool = ((self.accumulator ^ value) & NEGATIVE_BIT_MASK) == 0;

        self.set_accumulator(
            self.accumulator
                .wrapping_add(value)
                .wrapping_add(self.flag_check(FLAG_CARRY) as u8),
        );

        let overflow: bool =
            sign_bits_match && ((self.accumulator ^ value) & NEGATIVE_BIT_MASK) != 0;

        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_CARRY, self.accumulator < old_value);
        self.flag_toggle(FLAG_OVERFLOW, overflow);

        self.increment_program_counter(1);
    }

    pub(super) fn sbc(&mut self, address_mode: AddressingMode) {
        let old_value = self.accumulator;
        let operand = self.resolve_operand(address_mode);
        let value = match operand {
            OpcodeOperand::Byte(b) => b.wrapping_neg(),
            OpcodeOperand::Address(addr) => self.read_from_bus(addr).wrapping_neg(),
            _ => {
                panic!("Invalid addressing mode for ADC");
            }
        };
        self.increment_cycles(1);

        let sign_bits_match: bool = ((self.accumulator ^ value) & NEGATIVE_BIT_MASK) == 0;

        self.set_accumulator(
            self.accumulator
                .wrapping_add(value)
                .wrapping_add(self.flag_check(FLAG_CARRY) as u8),
        );

        let overflow: bool =
            sign_bits_match && ((self.accumulator ^ value) & NEGATIVE_BIT_MASK) != 0;

        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_CARRY, self.accumulator < old_value);
        self.flag_toggle(FLAG_OVERFLOW, overflow);

        self.increment_program_counter(1);
    }
}
