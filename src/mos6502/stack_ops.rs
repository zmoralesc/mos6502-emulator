use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn pha(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        let target_address: u16 = STACK_BASE + self.stack_pointer as u16;
        self.write_to_bus(target_address, self.accumulator)?;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn php(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        let target_address: u16 = STACK_BASE + self.stack_pointer as u16;
        self.write_to_bus(target_address, self.status_register | FLAG_BREAK | (1 << 5))?;
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn pla(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let target_address: u16 = STACK_BASE + ((self.stack_pointer as u16) & 0xFF);
        self.accumulator = self.read_from_bus(target_address)?;
        self.increment_cycles(4);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        Ok(())
    }

    pub(super) fn plp(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        let target_address: u16 = STACK_BASE + self.stack_pointer as u16;
        self.status_register = self.read_from_bus(target_address)?;
        self.status_register &= !FLAG_BREAK;
        self.status_register |= 1 << 5;
        self.increment_cycles(4);
        Ok(())
    }
}
