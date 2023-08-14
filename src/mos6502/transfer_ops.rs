use super::*;

impl<T: Bus> MOS6502<T> {
    // load value into accumulator
    pub(super) fn lda(&mut self, address_mode: AddressingMode) {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode);
        self.set_accumulator(match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.read_from_bus(addr),
            _ => {
                panic!("Invalid addressing mode for LDA");
            }
        });

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        self.increment_program_counter(1);
    }

    // load value into X register
    pub(super) fn ldx(&mut self, address_mode: AddressingMode) {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode);
        self.set_x_register(match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.read_from_bus(addr),
            _ => {
                panic!("Invalid addressing mode for LDX");
            }
        });

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT_MASK != 0);
        self.increment_program_counter(1);
    }

    // load value into Y register
    pub(super) fn ldy(&mut self, address_mode: AddressingMode) {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode);
        self.set_y_register(match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.read_from_bus(addr),
            _ => {
                panic!("Invalid addressing mode for LDY");
            }
        });

        self.flag_toggle(FLAG_ZERO, self.y_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.y_register & NEGATIVE_BIT_MASK != 0);
        self.increment_program_counter(1);
    }

    // store accumulator in memory
    pub(super) fn sta(&mut self, address_mode: AddressingMode) {
        let operand = self.resolve_operand(address_mode);
        let addr = match operand {
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for STA");
            }
        };
        self.increment_cycles(1);
        self.write_to_bus(addr, self.accumulator);
        self.increment_program_counter(1);
    }

    // store X register in memory
    pub(super) fn stx(&mut self, address_mode: AddressingMode) {
        let operand = self.resolve_operand(address_mode);
        let addr = match operand {
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for STX");
            }
        };
        self.increment_cycles(1);
        self.write_to_bus(addr, self.x_register);
        self.increment_program_counter(1);
    }

    // store Y register in memory
    pub(super) fn sty(&mut self, address_mode: AddressingMode) {
        let operand = self.resolve_operand(address_mode);
        let addr = match operand {
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for STY");
            }
        };
        self.increment_cycles(1);
        self.write_to_bus(addr, self.y_register);
        self.increment_program_counter(1);
    }

    // transfer accumulator to X register
    pub(super) fn tax(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.set_x_register(self.accumulator);

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
    }

    // transfer accumulator to Y register
    pub(super) fn tay(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.set_y_register(self.accumulator);

        self.flag_toggle(FLAG_ZERO, self.y_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.y_register & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
    }

    // transfer stack pointer to X register
    pub(super) fn tsx(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.set_x_register(self.stack_pointer);

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
    }

    // transfer X register to accumulator
    pub(super) fn txa(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.set_accumulator(self.x_register);

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
    }

    // transfer X register to stack pointer
    pub(super) fn txs(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.set_stack_pointer(self.x_register);

        self.increment_program_counter(1);
    }

    // transfer Y register to accumulator
    pub(super) fn tya(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
        self.set_accumulator(self.y_register);

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
    }
}
