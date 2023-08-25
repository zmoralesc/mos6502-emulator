use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn and(
        &mut self,
        address_mode: AddressingMode,
        bus: &mut T,
    ) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode, bus)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator &= operand;
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        Ok(())
    }

    pub(super) fn eor(
        &mut self,
        address_mode: AddressingMode,
        bus: &mut T,
    ) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode, bus)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator ^= operand;
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        Ok(())
    }

    pub(super) fn ora(
        &mut self,
        address_mode: AddressingMode,
        bus: &mut T,
    ) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode, bus)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator |= operand;
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        Ok(())
    }
}
