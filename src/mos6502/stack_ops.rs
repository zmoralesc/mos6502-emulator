use crate::{pop_from_stack, push_to_stack};

use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn pha(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        push_to_stack!(self, bus, self.accumulator);
        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn php(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        push_to_stack!(self, bus, self.status_register | FLAG_BREAK | (1 << 5));
        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn pla(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.accumulator = pop_from_stack!(self, bus);
        self.increment_cycles(4);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        Ok(())
    }

    pub(super) fn plp(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.status_register = pop_from_stack!(self, bus) | FLAG_BREAK | (1 << 5);
        self.increment_cycles(4);
        Ok(())
    }
}
