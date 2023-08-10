use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn and(&mut self, address_mode: AddressingMode) {
        let operand = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Word(w) => self.bus.read(w),
            _ => panic!("Invalid addressing mode for AND"),
        };
        self.accumulator = operand & self.accumulator;
    }

    pub(super) fn eor(&mut self, address_mode: AddressingMode) {
        let operand = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Word(w) => self.bus.read(w),
            _ => panic!("Invalid addressing mode for EOR"),
        };
        self.accumulator = operand ^ self.accumulator;
    }

    pub(super) fn ora(&mut self, address_mode: AddressingMode) {
        let operand = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Word(w) => self.bus.read(w),
            _ => panic!("Invalid addressing mode for ORA"),
        };
        self.accumulator = operand | self.accumulator;
    }
}