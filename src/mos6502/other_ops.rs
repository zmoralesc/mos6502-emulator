use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn nop(&mut self, _: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn bit(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };

        self.flag_toggle(FLAG_NEGATIVE, operand & (1 << 7) != 0);
        self.flag_toggle(FLAG_OVERFLOW, operand & (1 << 6) != 0);
        self.flag_toggle(FLAG_ZERO, operand & self.accumulator == 0);

        self.increment_cycles(1);
        Ok(())
    }
}
