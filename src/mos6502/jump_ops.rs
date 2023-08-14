use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn jmp(&mut self, address_mode: AddressingMode) {
        let new_pc_value = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode for JMP"),
        };
        self.set_program_counter(new_pc_value);
        self.increment_cycles(1);
    }

    pub(super) fn jsr(&mut self, address_mode: AddressingMode) {
        let return_address = self.program_counter + 2;

        let return_address_lo: u8 = (return_address & 0xFF) as u8;
        let return_address_hi: u8 = ((return_address >> 8) & 0xFF) as u8;

        self.write_to_bus(STACK_BASE + self.stack_pointer as u16, return_address_lo);
        self.set_stack_pointer(self.stack_pointer.wrapping_sub(1));
        self.write_to_bus(STACK_BASE + self.stack_pointer as u16, return_address_hi);
        self.set_stack_pointer(self.stack_pointer.wrapping_sub(1));

        let new_pc_value = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode for JSR"),
        };
        self.set_program_counter(new_pc_value);
        self.increment_cycles(6);
    }

    pub(super) fn rts(&mut self, _: AddressingMode) {
        let return_address_hi = self.read_from_bus(STACK_BASE + self.stack_pointer as u16);
        self.set_stack_pointer(self.stack_pointer.wrapping_add(1));

        let return_address_lo = self.read_from_bus(STACK_BASE + self.stack_pointer as u16);
        self.set_stack_pointer(self.stack_pointer.wrapping_add(1));

        self.set_program_counter(u16::from_le_bytes([return_address_lo, return_address_hi]));
        self.increment_cycles(6);
    }
}
