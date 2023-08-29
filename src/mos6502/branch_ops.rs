use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn bcc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        if !self.flag_check(FLAG_CARRY) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bcs(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        if self.flag_check(FLAG_CARRY) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn beq(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        if self.flag_check(FLAG_ZERO) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bmi(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        if self.flag_check(FLAG_NEGATIVE) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bne(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        if !self.flag_check(FLAG_ZERO) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bpl(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        if !self.flag_check(FLAG_NEGATIVE) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bvc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        if !self.flag_check(FLAG_OVERFLOW) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn bvs(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        if self.flag_check(FLAG_OVERFLOW) {
            self.set_program_counter(addr);
        }
        self.increment_cycles(1);
        Ok(())
    }
}
