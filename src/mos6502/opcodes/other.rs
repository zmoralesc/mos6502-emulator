use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn nop(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        Ok(2)
    }

    pub(in crate::mos6502) fn bit(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };

        self.flag_set(CpuFlags::Negative, operand & (1 << 7) != 0);
        self.flag_set(CpuFlags::Overflow, operand & (1 << 6) != 0);
        self.flag_set(CpuFlags::Zero, operand & self.accumulator == 0);

        Ok(0)
    }
}
