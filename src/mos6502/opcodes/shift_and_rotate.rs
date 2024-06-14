use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn asl(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        match operand {
            OpcodeOperand::Byte(_) => {
                self.flag_set(CpuFlags::Carry, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.accumulator = self.accumulator.wrapping_shl(1);
                self.flag_set(
                    CpuFlags::Negative,
                    self.accumulator & NEGATIVE_BIT_MASK != 0,
                );
                self.flag_set(CpuFlags::Zero, self.accumulator == 0);
            }
            OpcodeOperand::Address(w) => {
                let mut value = bus.read(w)?;
                self.flag_set(CpuFlags::Carry, value & NEGATIVE_BIT_MASK != 0);
                value = value.wrapping_shl(1);
                self.flag_set(CpuFlags::Negative, value & NEGATIVE_BIT_MASK != 0);
                self.flag_set(CpuFlags::Zero, value == 0);
                bus.write(w, value)?;
            }
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn lsr(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        match operand {
            OpcodeOperand::Byte(_) => {
                let bit0_is_set = self.accumulator & 1 != 0;
                self.accumulator = self.accumulator.wrapping_shr(1);
                self.flag_set(CpuFlags::Zero, self.accumulator == 0);
                self.flag_set(CpuFlags::Carry, bit0_is_set);
            }
            OpcodeOperand::Address(w) => {
                let mut value = bus.read(w)?;
                let bit0_is_set = value & 1 != 0;
                value = value.wrapping_shr(1);
                self.flag_set(CpuFlags::Zero, value == 0);
                self.flag_set(CpuFlags::Carry, bit0_is_set);
                bus.write(w, value)?;
            }
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.flag_set(CpuFlags::Negative, false);
        Ok(cycles)
    }

    pub(in crate::mos6502) fn rol(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let carry_bit_mask = self.flag_check(CpuFlags::Carry) as u8;
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        match operand {
            OpcodeOperand::Byte(_) => {
                let bit7_is_set = self.accumulator & NEGATIVE_BIT_MASK != 0;
                self.accumulator = self.accumulator.wrapping_shl(1) | carry_bit_mask;
                self.flag_set(CpuFlags::Carry, bit7_is_set);
                self.flag_set(CpuFlags::Zero, self.accumulator == 0);
                self.flag_set(
                    CpuFlags::Negative,
                    self.accumulator & NEGATIVE_BIT_MASK != 0,
                );
            }
            OpcodeOperand::Address(w) => {
                let value = bus.read(w)?;
                let bit7_is_set = value & NEGATIVE_BIT_MASK != 0;
                let new_value: u8 = value.wrapping_shl(1) | carry_bit_mask;
                self.flag_set(CpuFlags::Carry, bit7_is_set);
                self.flag_set(CpuFlags::Zero, new_value == 0);
                self.flag_set(CpuFlags::Negative, new_value & NEGATIVE_BIT_MASK != 0);
                bus.write(w, new_value)?;
            }
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        }
        Ok(cycles)
    }

    pub(in crate::mos6502) fn ror(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let carry_bit_mask = (self.flag_check(CpuFlags::Carry) as u8) << 7;
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        match operand {
            OpcodeOperand::Byte(_) => {
                let bit0_is_set = self.accumulator & 1 == 1;
                self.accumulator = self.accumulator.wrapping_shr(1) | carry_bit_mask;
                self.flag_set(CpuFlags::Carry, bit0_is_set);
                self.flag_set(CpuFlags::Zero, self.accumulator == 0);
                self.flag_set(
                    CpuFlags::Negative,
                    self.accumulator & NEGATIVE_BIT_MASK != 0,
                );
            }
            OpcodeOperand::Address(w) => {
                let value = bus.read(w)?;
                let bit0_is_set = value & 1 == 1;
                let new_value: u8 = value.wrapping_shr(1) | carry_bit_mask;
                self.flag_set(CpuFlags::Carry, bit0_is_set);
                self.flag_set(CpuFlags::Zero, new_value == 0);
                self.flag_set(CpuFlags::Negative, new_value & NEGATIVE_BIT_MASK != 0);
                bus.write(w, new_value)?;
            }
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        }
        Ok(cycles)
    }
}
