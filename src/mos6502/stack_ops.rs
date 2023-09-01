use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn pha(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.push_to_stack(bus, self.accumulator)?;
        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn php(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.push_to_stack(
            bus,
            self.status_register | (CpuFlags::Break | CpuFlags::Unused).as_u8(),
        )?;
        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn pla(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.accumulator = self.pop_from_stack(bus)?;
        self.increment_cycles(4);
        self.flag_toggle(CpuFlags::Zero, self.accumulator == 0);
        self.flag_toggle(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        Ok(())
    }

    pub(super) fn plp(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.status_register =
            self.pop_from_stack(bus)? | (CpuFlags::Break | CpuFlags::Unused).as_u8();
        self.increment_cycles(4);
        Ok(())
    }
}
