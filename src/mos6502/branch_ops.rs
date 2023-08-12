use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn bcc(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode."),
        };
        if !self.flag_check(FLAG_CARRY) {
            self.program_counter = addr;
        }
        self.increment_cycles(1);
    }

    pub(super) fn bcs(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode."),
        };
        if self.flag_check(FLAG_CARRY) {
            self.program_counter = addr;
        }
        self.increment_cycles(1);
    }

    pub(super) fn beq(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode."),
        };
        if self.flag_check(FLAG_ZERO) {
            self.program_counter = addr;
        }
        self.increment_cycles(1);
    }

    pub(super) fn bmi(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode."),
        };
        if self.flag_check(FLAG_NEGATIVE) {
            self.program_counter = addr;
        }
        self.increment_cycles(1);
    }

    pub(super) fn bne(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode."),
        };
        if !self.flag_check(FLAG_ZERO) {
            self.program_counter = addr;
        }
        self.increment_cycles(1);
    }

    pub(super) fn bpl(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode."),
        };
        if !self.flag_check(FLAG_NEGATIVE) {
            self.program_counter = addr;
        }
        self.increment_cycles(1);
    }

    pub(super) fn bvc(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode."),
        };
        if !self.flag_check(FLAG_OVERFLOW) {
            self.program_counter = addr;
        }
        self.increment_cycles(1);
    }

    pub(super) fn bvs(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Address(w) => w,
            _ => panic!("Invalid addressing mode."),
        };
        if self.flag_check(FLAG_OVERFLOW) {
            self.program_counter = addr;
        }
        self.increment_cycles(1);
    }
}
