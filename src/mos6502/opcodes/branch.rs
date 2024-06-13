use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn bcc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        if !self.flag_check(CpuFlags::Carry) {
            self.set_program_counter(addr);
        }
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn bcs(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        if self.flag_check(CpuFlags::Carry) {
            self.set_program_counter(addr);
        }
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn beq(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        if self.flag_check(CpuFlags::Zero) {
            self.set_program_counter(addr);
        }
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn bmi(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        if self.flag_check(CpuFlags::Negative) {
            self.set_program_counter(addr);
        }
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn bne(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        if !self.flag_check(CpuFlags::Zero) {
            self.set_program_counter(addr);
        }
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn bpl(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        if !self.flag_check(CpuFlags::Negative) {
            self.set_program_counter(addr);
        }
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn bvc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        if !self.flag_check(CpuFlags::Overflow) {
            self.set_program_counter(addr);
        }
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn bvs(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        if self.flag_check(CpuFlags::Overflow) {
            self.set_program_counter(addr);
        }
        Ok(cycles + 1)
    }
}
