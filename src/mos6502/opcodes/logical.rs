use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn and(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.accumulator &= operand;
        self.flag_set(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        self.flag_set(CpuFlags::Zero, self.accumulator == 0);
        Ok(())
    }

    pub(in crate::mos6502) fn eor(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.accumulator ^= operand;
        self.flag_set(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        self.flag_set(CpuFlags::Zero, self.accumulator == 0);
        Ok(())
    }

    pub(in crate::mos6502) fn ora(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.accumulator |= operand;
        self.flag_set(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        self.flag_set(CpuFlags::Zero, self.accumulator == 0);
        Ok(())
    }
}