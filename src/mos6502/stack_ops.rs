use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn pha(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.push_to_stack(bus, self.accumulator)?;
        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn php(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.push_to_stack(bus, self.status_register | FLAG_BREAK | (1 << 5))?;
        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn pla(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.accumulator = self.pop_from_stack(bus)?;
        self.increment_cycles(4);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        Ok(())
    }

    pub(super) fn plp(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.status_register = self.pop_from_stack(bus)? | FLAG_BREAK | (1 << 5);
        self.increment_cycles(4);
        Ok(())
    }
}
