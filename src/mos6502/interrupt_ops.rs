use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn brk(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        let res = self.perform_interrupt(self.program_counter + 1, InterruptKind::Irq, bus);
        self.flag_toggle(FLAG_NO_INTERRUPTS, true);
        res
    }

    pub(super) fn rti(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.status_register = self.pop_from_stack(bus)? | FLAG_BREAK | (1 << 5);
        let return_address_lo = self.pop_from_stack(bus)?;
        let return_address_hi = self.pop_from_stack(bus)?;

        self.set_program_counter(u16::from_le_bytes([return_address_lo, return_address_hi]));
        self.increment_cycles(6);
        Ok(())
    }
}
