use super::*;

impl<T: Bus + Send + Sync> MOS6502<T> {
    pub(super) fn and(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator &= operand;
        self.increment_program_counter(1);
        Ok(())
    }

    pub(super) fn eor(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator ^= operand;
        self.increment_program_counter(1);
        Ok(())
    }

    pub(super) fn ora(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator |= operand;
        self.increment_program_counter(1);
        Ok(())
    }
}
