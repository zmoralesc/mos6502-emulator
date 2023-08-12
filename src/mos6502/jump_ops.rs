use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn jmp(&mut self, address_mode: AddressingMode) {
        self.program_counter = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode for JMP"),
        };
        self.increment_cycles(1);
    }

    pub(super) fn jsr(&mut self, address_mode: AddressingMode) {
        let return_address = self.program_counter + 2;

        let return_address_lo: u8 = (return_address & 0xFF) as u8;
        let return_address_hi: u8 = ((return_address >> 8) & 0xFF) as u8;

        self.bus
            .write(STACK_BASE + self.stack_pointer as u16, return_address_lo);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        self.bus
            .write(STACK_BASE + self.stack_pointer as u16, return_address_hi);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);

        self.program_counter = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode for JSR"),
        };
        self.increment_cycles(6);
    }

    pub(super) fn rts(&mut self, _: AddressingMode) {
        let return_address_hi = self.bus.read(STACK_BASE + self.stack_pointer as u16);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        let return_address_lo = self.bus.read(STACK_BASE + self.stack_pointer as u16);
        self.stack_pointer = self.stack_pointer.wrapping_add(1);

        self.program_counter = u16::from_le_bytes([return_address_lo, return_address_hi]);
        self.increment_cycles(6);
    }
}
