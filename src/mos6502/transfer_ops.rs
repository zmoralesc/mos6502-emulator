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
    };
}

macro_rules! load_value_to_register {
    ($cpu:expr, $register:expr, $address_mode:ident) => {
        $cpu.increment_cycles(1);
        let operand = $cpu.resolve_operand($address_mode)?;
        $register = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => $cpu.read_from_bus(addr)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };

        $cpu.flag_toggle(FLAG_ZERO, $register == 0);
        $cpu.flag_toggle(FLAG_NEGATIVE, $register & NEGATIVE_BIT_MASK != 0);
    };
}

macro_rules! transfer_register {
    ($cpu:expr, $source_register:expr, $target_register:expr) => {
        $cpu.increment_cycles(2);
        $target_register = $source_register;

        $cpu.flag_toggle(FLAG_ZERO, $source_register == 0);
        $cpu.flag_toggle(FLAG_NEGATIVE, $source_register & NEGATIVE_BIT_MASK != 0);
    };
}

impl<T: Bus> MOS6502<T> {
    // load value into accumulator
    pub(super) fn lda(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        load_value_to_register!(self, self.accumulator, address_mode);
        Ok(())
    }

    // load value into X register
    pub(super) fn ldx(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        load_value_to_register!(self, self.x_register, address_mode);
        Ok(())
    }

    // load value into Y register
    pub(super) fn ldy(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        load_value_to_register!(self, self.y_register, address_mode);
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
        transfer_register!(self, self.accumulator, self.x_register);
        Ok(())
    }

    // transfer accumulator to Y register
    pub(super) fn tay(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        transfer_register!(self, self.accumulator, self.y_register);
        Ok(())
    }

    // transfer stack pointer to X register
    pub(super) fn tsx(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        transfer_register!(self, self.stack_pointer, self.x_register);
        Ok(())
    }

    // transfer X register to accumulator
    pub(super) fn txa(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        transfer_register!(self, self.x_register, self.accumulator);
        Ok(())
    }

    // transfer X register to stack pointer
    pub(super) fn txs(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.increment_cycles(2);
        self.stack_pointer = self.x_register;

        Ok(())
    }

    // transfer Y register to accumulator
    pub(super) fn tya(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        transfer_register!(self, self.y_register, self.accumulator);
        Ok(())
    }
}
