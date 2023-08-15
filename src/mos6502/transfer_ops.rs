use super::*;

macro_rules! store_register_value {
    ($cpu:expr, $register:expr, $address_mode:ident) => {
        let operand = $cpu.resolve_operand($address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(addr) => addr,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        $cpu.increment_cycles(1);
        $cpu.write_to_bus(addr, $register)?;
        $cpu.increment_program_counter(1);
    };
}

impl<T: Bus + Send + Sync> MOS6502<T> {
    // load value into accumulator
    pub(super) fn lda(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode)?;
        self.set_accumulator(match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.read_from_bus(addr)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        });

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        self.increment_program_counter(1);
        Ok(())
    }

    // load value into X register
    pub(super) fn ldx(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode)?;
        self.set_x_register(match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.read_from_bus(addr)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        });

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT_MASK != 0);
        self.increment_program_counter(1);
        Ok(())
    }

    // load value into Y register
    pub(super) fn ldy(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(1);
        let operand = self.resolve_operand(address_mode)?;
        self.set_y_register(match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.read_from_bus(addr)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        });

        self.flag_toggle(FLAG_ZERO, self.y_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.y_register & NEGATIVE_BIT_MASK != 0);
        self.increment_program_counter(1);
        Ok(())
    }

    // store accumulator in memory
    pub(super) fn sta(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        store_register_value!(self, self.accumulator, address_mode);
        Ok(())
    }

    // store X register in memory
    pub(super) fn stx(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        store_register_value!(self, self.x_register, address_mode);
        Ok(())
    }

    // store Y register in memory
    pub(super) fn sty(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        store_register_value!(self, self.y_register, address_mode);
        Ok(())
    }

    // transfer accumulator to X register
    pub(super) fn tax(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(2);
        self.set_x_register(self.accumulator);

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
        Ok(())
    }

    // transfer accumulator to Y register
    pub(super) fn tay(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(2);
        self.set_y_register(self.accumulator);

        self.flag_toggle(FLAG_ZERO, self.y_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.y_register & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
        Ok(())
    }

    // transfer stack pointer to X register
    pub(super) fn tsx(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(2);
        self.set_x_register(self.stack_pointer);

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
        Ok(())
    }

    // transfer X register to accumulator
    pub(super) fn txa(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(2);
        self.set_accumulator(self.x_register);

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
        Ok(())
    }

    // transfer X register to stack pointer
    pub(super) fn txs(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(2);
        self.set_stack_pointer(self.x_register);

        self.increment_program_counter(1);
        Ok(())
    }

    // transfer Y register to accumulator
    pub(super) fn tya(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(2);
        self.set_accumulator(self.y_register);

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);

        self.increment_program_counter(1);
        Ok(())
    }
}
