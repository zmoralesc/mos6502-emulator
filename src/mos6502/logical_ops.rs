use super::*;

impl<T: Bus + Send + Sync> MOS6502<T> {
    pub(super) fn and(&mut self, address_mode: AddressingMode) {
        let operand = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w),
            _ => panic!("Invalid addressing mode for AND"),
        };
        self.set_accumulator(operand & self.accumulator);
        self.increment_program_counter(1);
    }

    pub(super) fn eor(&mut self, address_mode: AddressingMode) {
        let operand = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w),
            _ => panic!("Invalid addressing mode for EOR"),
        };
        self.set_accumulator(operand ^ self.accumulator);
        self.increment_program_counter(1);
    }

    pub(super) fn ora(&mut self, address_mode: AddressingMode) {
        let operand = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => self.read_from_bus(w),
            _ => panic!("Invalid addressing mode for ORA"),
        };
        self.set_accumulator(operand | self.accumulator);
        self.increment_program_counter(1);
    }
}
