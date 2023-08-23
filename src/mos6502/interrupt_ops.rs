use crate::pop_from_stack;

use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn brk(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.flag_toggle(FLAG_NO_INTERRUPTS, true);
        self.perform_interrupt(self.program_counter + 1, InterruptKind::Irq)
    }

    pub(super) fn rti(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        // pop SR from stack
        self.status_register = pop_from_stack!(self) | FLAG_BREAK | (1 << 5);

        // pop low byte of return address from stack
        let return_address_lo = pop_from_stack!(self);

        // pop high byte of return address from stack
        let return_address_hi = pop_from_stack!(self);

        self.set_program_counter(u16::from_le_bytes([return_address_lo, return_address_hi]));
        self.increment_cycles(6);
        Ok(())
    }
}
