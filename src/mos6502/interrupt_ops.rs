use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn brk(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.perform_interrupt(self.program_counter + 2, InterruptKind::IRQ)
    }

    pub(super) fn rti(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        // pop SR from stack
        self.status_register =
            self.read_from_bus(STACK_BASE + self.stack_pointer as u16)? & !FLAG_BREAK;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        // pop low byte of return address from stack
        let return_address_lo = self.read_from_bus(STACK_BASE + self.stack_pointer as u16)?;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        // pop high byte of return address from stack
        let return_address_hi = self.read_from_bus(STACK_BASE + self.stack_pointer as u16)?;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        self.set_program_counter(u16::from_le_bytes([return_address_lo, return_address_hi]));
        self.increment_cycles(6);
        Ok(())
    }
}
