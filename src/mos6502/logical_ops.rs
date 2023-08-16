use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn and(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator &= operand;
        Ok(())
    }

    pub(super) fn eor(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator ^= operand;
        Ok(())
    }

    pub(super) fn ora(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let operand = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.accumulator |= operand;
        Ok(())
    }
}
