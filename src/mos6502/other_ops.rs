use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn nop(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn bit(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(CpuError::InvalidAddressingMode),
        };

        self.flag_toggle(CpuFlags::Negative, operand & (1 << 7) != 0);
        self.flag_toggle(CpuFlags::Overflow, operand & (1 << 6) != 0);
        self.flag_toggle(CpuFlags::Zero, operand & self.accumulator == 0);

        self.increment_cycles(1);
        Ok(())
    }
}
