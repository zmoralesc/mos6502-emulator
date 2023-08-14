use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn nop(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
    }

    pub(super) fn bit(&mut self, address_mode: AddressingMode) {
        let operand = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => self.read_from_bus(w),
            _ => panic!("Invalid addressing mode."),
        };

        self.flag_toggle(FLAG_NEGATIVE, operand & (1 << 7) != 0);
        self.flag_toggle(FLAG_OVERFLOW, operand & (1 << 6) != 0);
        self.flag_toggle(FLAG_ZERO, operand & self.accumulator != 0);

        self.increment_cycles(1);
    }
}
