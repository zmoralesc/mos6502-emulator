use super::*;

impl<T: Bus> MOS6502<T> {
    // load value into accumulator
    pub(super) fn lda(&mut self, address_mode: AddressingMode) {
        self.cycles = self.cycles.wrapping_add(1);
        let operand = self.resolve_operand(address_mode);
        self.accumulator = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDA");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT != 0);

        self.program_counter = self.program_counter.wrapping_add(1);
    }

    // load value into X register
    pub(super) fn ldx(&mut self, address_mode: AddressingMode) {
        self.cycles = self.cycles.wrapping_add(1);
        let operand = self.resolve_operand(address_mode);
        self.x_register = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDX");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT != 0);

        self.program_counter = self.program_counter.wrapping_add(1);
    }

    // load value into Y register
    pub(super) fn ldy(&mut self, address_mode: AddressingMode) {
        self.cycles = self.cycles.wrapping_add(1);
        let operand = self.resolve_operand(address_mode);
        self.y_register = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDY");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.y_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.y_register & NEGATIVE_BIT != 0);

        self.program_counter = self.program_counter.wrapping_add(1);
    }

    // add to accumulator with carry
    pub(super) fn adc(&mut self, address_mode: AddressingMode) {
        let a_oldvalue = self.accumulator;
        let operand = self.resolve_operand(address_mode);
        let value = match operand {
            OpcodeOperand::Byte(b) => self.bus.read(b as u16),
            _ => {
                panic!("Invalid addressing mode for ADC");
            }
        };

        self.accumulator = self
            .accumulator
            .wrapping_add(value)
            .wrapping_add(self.flag_check(FLAG_CARRY) as u8);

        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_CARRY, self.accumulator < a_oldvalue);
        self.flag_toggle(FLAG_OVERFLOW, true);
    }

    pub(super) fn not_implemented(&mut self, _: AddressingMode) {
        panic!("Opcode not implemented.")
    }
}
