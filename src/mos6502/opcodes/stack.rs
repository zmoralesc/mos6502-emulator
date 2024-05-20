use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn pha(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<(), CpuError> {
        self.push_to_stack(bus, self.accumulator)?;
        self.increment_cycles(3);
        Ok(())
    }

    pub(in crate::mos6502) fn php(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<(), CpuError> {
        self.push_to_stack(
            bus,
            (self.status_register | CpuFlags::Break | CpuFlags::Unused).into(),
        )?;
        self.increment_cycles(3);
        Ok(())
    }

    pub(in crate::mos6502) fn pla(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<(), CpuError> {
        self.accumulator = self.pop_from_stack(bus)?;
        self.increment_cycles(4);
        self.flag_toggle(CpuFlags::Zero, self.accumulator == 0);
        self.flag_toggle(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        Ok(())
    }

    pub(in crate::mos6502) fn plp(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<(), CpuError> {
        self.status_register =
            CpuFlags::from(self.pop_from_stack(bus)?) | CpuFlags::Break | CpuFlags::Unused;
        self.increment_cycles(4);
        Ok(())
    }
}
