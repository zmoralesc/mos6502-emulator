use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn pha(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.push_to_stack(bus, self.accumulator)?;
        Ok(3)
    }

    pub(in crate::mos6502) fn php(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.push_to_stack(
            bus,
            (self.status_register | CpuFlags::Break | CpuFlags::Unused).into(),
        )?;
        Ok(3)
    }

    pub(in crate::mos6502) fn pla(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.accumulator = self.pop_from_stack(bus)?;
        self.flag_set(CpuFlags::Zero, self.accumulator == 0);
        self.flag_set(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        Ok(4)
    }

    pub(in crate::mos6502) fn plp(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.status_register =
            CpuFlags::from(self.pop_from_stack(bus)?) | CpuFlags::Break | CpuFlags::Unused;
        Ok(4)
    }
}
