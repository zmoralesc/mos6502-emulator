use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn brk(&mut self, address_mode: AddressingMode) {
        let return_address = self.program_counter + 2;

        let return_address_lo: u8 = (return_address & 0xFF) as u8;
        let return_address_hi: u8 = ((return_address >> 8) & 0xFF) as u8;

        // push low byte of return address to stack
        self.bus
            .write(STACK_BASE + self.stack_pointer as u16, return_address_lo);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        // push high byte of return address to stack
        self.bus
            .write(STACK_BASE + self.stack_pointer as u16, return_address_hi);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        // push SR to stack
        self.bus.write(
            STACK_BASE + self.stack_pointer as u16,
            self.status_register | FLAG_BREAK,
        );
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        self.flag_toggle(FLAG_NO_INTERRUPTS, true);

        let divert_address_lo = self.bus.read(0xFFFE);
        let divert_address_hi = self.bus.read(0xFFFF);

        self.program_counter = u16::from_le_bytes([divert_address_lo, divert_address_hi]);

        self.increment_cycles(7);
    }

    pub(super) fn rti(&mut self, address_mode: AddressingMode) {
        // pull SR from stack
        self.status_register = self.bus.read(STACK_BASE + self.stack_pointer as u16) & !FLAG_BREAK;
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        // pull high byte of return address from stack
        let return_address_hi = self.bus.read(STACK_BASE + self.stack_pointer as u16);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        // pull low byte of return address from stack
        let return_address_lo = self.bus.read(STACK_BASE + self.stack_pointer as u16);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        self.program_counter = u16::from_le_bytes([return_address_lo, return_address_hi]);
        self.increment_cycles(6);
    }
}
