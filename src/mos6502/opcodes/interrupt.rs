use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn brk(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.perform_interrupt(self.program_counter + 1, InterruptKind::Brk, bus)
    }

    pub(in crate::mos6502) fn rti(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.status_register =
            CpuFlags::from(self.pop_from_stack(bus)?) | CpuFlags::Break | CpuFlags::Unused;
        let return_address_lo = self.pop_from_stack(bus)?;
        let return_address_hi = self.pop_from_stack(bus)?;

        self.set_program_counter(u16::from_le_bytes([return_address_lo, return_address_hi]));
        Ok(6)
    }
}
