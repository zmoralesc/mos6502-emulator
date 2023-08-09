use super::*;

impl<T: Bus> MOS6502<T> {
    // load value into accumulator
    pub(super) fn lda(&mut self, address_mode: AddressingMode) {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode);
        self.accumulator = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDA");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & SIGN_BIT_MASK != 0);
    }

    // load value into X register
    pub(super) fn ldx(&mut self, address_mode: AddressingMode) {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode);
        self.x_register = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDX");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & SIGN_BIT_MASK != 0);
    }

    // load value into Y register
    pub(super) fn ldy(&mut self, address_mode: AddressingMode) {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode);
        self.y_register = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDY");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.y_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.y_register & SIGN_BIT_MASK != 0);
    }

    // store accumulator in memory
    pub(super) fn sta(&mut self, address_mode: AddressingMode) {
        let operand = self.resolve_operand(address_mode);
        self.increment_cycles(1);
        let addr = match operand {
            OpcodeOperand::Byte(b) => b as u16,
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for STA");
            }
        };
        self.bus.write(addr, self.accumulator);
    }

    // store X register in memory
    pub(super) fn stx(&mut self, address_mode: AddressingMode) {
        let operand = self.resolve_operand(address_mode);
        self.increment_cycles(1);
        let addr = match operand {
            OpcodeOperand::Byte(b) => b as u16,
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for STX");
            }
        };
        self.bus.write(addr, self.x_register);
    }

    // store Y register in memory
    pub(super) fn sty(&mut self, address_mode: AddressingMode) {
        let operand = self.resolve_operand(address_mode);
        self.increment_cycles(1);
        let addr = match operand {
            OpcodeOperand::Byte(b) => b as u16,
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for STX");
            }
        };
        self.bus.write(addr, self.y_register);
    }

    // transfer accumulator to X register
    pub(super) fn tax(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.x_register = self.accumulator;

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & SIGN_BIT_MASK != 0);
    }

    // transfer accumulator to Y register
    pub(super) fn tay(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.y_register = self.accumulator;

        self.flag_toggle(FLAG_ZERO, self.y_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.y_register & SIGN_BIT_MASK != 0);
    }

    // transfer stack pointer to X register
    pub(super) fn tsx(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.x_register = self.stack_pointer;

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & SIGN_BIT_MASK != 0);
    }

    // transfer X register to accumulator
    pub(super) fn txa(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.accumulator = self.x_register;

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & SIGN_BIT_MASK != 0);
    }

    // transfer X register to stack pointer
    pub(super) fn txs(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.stack_pointer = self.x_register;
    }

    // transfer Y register to accumulator
    pub(super) fn tya(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.accumulator = self.y_register;

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & SIGN_BIT_MASK != 0);
    }
}
