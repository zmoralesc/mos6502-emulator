use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn pha(&mut self, _: AddressingMode) {
        let target_address: u16 = STACK_BASE + self.stack_pointer as u16;
        self.write_to_bus(target_address, self.accumulator);
        self.set_stack_pointer(self.stack_pointer.wrapping_sub(1));
        self.increment_cycles(3);
    }

    pub(super) fn php(&mut self, _: AddressingMode) {
        let target_address: u16 = STACK_BASE + self.stack_pointer as u16;
        self.write_to_bus(
            target_address,
            self.status_register | FLAG_BREAK | FLAG_NO_INTERRUPTS,
        );
        self.set_stack_pointer(self.stack_pointer.wrapping_sub(1));
        self.increment_cycles(3);
    }

    pub(super) fn pla(&mut self, _: AddressingMode) {
        let target_address: u16 = STACK_BASE + self.stack_pointer as u16;
        self.set_accumulator(self.read_from_bus(target_address));
        self.set_stack_pointer(self.stack_pointer.wrapping_add(1));
        self.increment_cycles(4);
    }

    pub(super) fn plp(&mut self, _: AddressingMode) {
        let target_address: u16 = STACK_BASE + self.stack_pointer as u16;
        self.set_status_register(self.read_from_bus(target_address) & !FLAG_BREAK & !FLAG_NO_INTERRUPTS);
        self.set_stack_pointer(self.stack_pointer.wrapping_add(1));
        self.increment_cycles(4);
    }
}
