use crate::pop_from_stack;

use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn brk(&mut self, _: AddressingMode, bus: &mut T) -> Result<(), EmulationError> {
        let res = self.perform_interrupt(self.program_counter + 1, InterruptKind::Irq, bus);
        self.flag_toggle(FLAG_NO_INTERRUPTS, true);
        res
    }

    pub(super) fn rti(&mut self, _: AddressingMode, bus: &mut T) -> Result<(), EmulationError> {
        self.status_register = pop_from_stack!(self, bus) | FLAG_BREAK | (1 << 5);
        let return_address_lo = pop_from_stack!(self, bus);
        let return_address_hi = pop_from_stack!(self, bus);

        self.set_program_counter(u16::from_le_bytes([return_address_lo, return_address_hi]));
        self.increment_cycles(6);
        Ok(())
    }
}
