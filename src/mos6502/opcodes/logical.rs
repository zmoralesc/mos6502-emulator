use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn and(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let mut extra_cycles = 0;
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            OpcodeOperand::AddressWithOverflow(addr, overflow) => {
                extra_cycles += overflow as u32;
                bus.read(addr)?
            }
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.accumulator &= operand;
        self.flag_set(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        self.flag_set(CpuFlags::Zero, self.accumulator == 0);
        Ok(extra_cycles)
    }

    pub(in crate::mos6502) fn eor(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let mut extra_cycles = 0;
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            OpcodeOperand::AddressWithOverflow(addr, overflow) => {
                extra_cycles += overflow as u32;
                bus.read(addr)?
            }
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.accumulator ^= operand;
        self.flag_set(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        self.flag_set(CpuFlags::Zero, self.accumulator == 0);
        Ok(extra_cycles)
    }

    pub(in crate::mos6502) fn ora(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let mut extra_cycles = 0;
        let operand = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            OpcodeOperand::AddressWithOverflow(addr, overflow) => {
                extra_cycles += overflow as u32;
                bus.read(addr)?
            }
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.accumulator |= operand;
        self.flag_set(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        self.flag_set(CpuFlags::Zero, self.accumulator == 0);
        Ok(extra_cycles)
    }
}
