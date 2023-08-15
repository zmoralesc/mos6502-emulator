use super::*;

impl<T: Bus + Send + Sync> MOS6502<T> {
    pub(super) fn bcc(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        if !self.flag_check(FLAG_CARRY) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bcs(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        if self.flag_check(FLAG_CARRY) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn beq(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        if self.flag_check(FLAG_ZERO) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bmi(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        if self.flag_check(FLAG_NEGATIVE) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bne(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        if !self.flag_check(FLAG_ZERO) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bpl(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        if !self.flag_check(FLAG_NEGATIVE) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bvc(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        if !self.flag_check(FLAG_OVERFLOW) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bvs(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        if self.flag_check(FLAG_OVERFLOW) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }
}
