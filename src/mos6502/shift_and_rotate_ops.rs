use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn asl(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(_) => {
                self.flag_toggle(CpuFlags::Carry, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.accumulator = self.accumulator.wrapping_shl(1);
                self.flag_toggle(
                    CpuFlags::Negative,
                    self.accumulator & NEGATIVE_BIT_MASK != 0,
                );
                self.flag_toggle(CpuFlags::Zero, self.accumulator == 0);
            }
            OpcodeOperand::Address(w) => {
                let mut value = bus.read(w)?;
                self.flag_toggle(CpuFlags::Carry, value & NEGATIVE_BIT_MASK != 0);
                value = value.wrapping_shl(1);
                self.flag_toggle(CpuFlags::Negative, value & NEGATIVE_BIT_MASK != 0);
                self.flag_toggle(CpuFlags::Zero, value == 0);
                bus.write(w, value)?;
            }
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        Ok(())
    }

    pub(super) fn lsr(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(_) => {
                let bit0_is_set = self.accumulator & 1 != 0;
                self.accumulator = self.accumulator.wrapping_shr(1);
                self.flag_toggle(CpuFlags::Zero, self.accumulator == 0);
                self.flag_toggle(CpuFlags::Carry, bit0_is_set);
            }
            OpcodeOperand::Address(w) => {
                let mut value = bus.read(w)?;
                let bit0_is_set = value & 1 != 0;
                value = value.wrapping_shr(1);
                self.flag_toggle(CpuFlags::Zero, value == 0);
                self.flag_toggle(CpuFlags::Carry, bit0_is_set);
                bus.write(w, value)?;
            }
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        self.flag_toggle(CpuFlags::Negative, false);
        Ok(())
    }

    pub(super) fn rol(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let carry_bit_mask = self.flag_check(CpuFlags::Carry) as u8;
        match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(_) => {
                let bit7_is_set = self.accumulator & NEGATIVE_BIT_MASK != 0;
                self.accumulator = self.accumulator.wrapping_shl(1) | carry_bit_mask;
                self.flag_toggle(CpuFlags::Carry, bit7_is_set);
                self.flag_toggle(CpuFlags::Zero, self.accumulator == 0);
                self.flag_toggle(
                    CpuFlags::Negative,
                    self.accumulator & NEGATIVE_BIT_MASK != 0,
                );
            }
            OpcodeOperand::Address(w) => {
                let value = bus.read(w)?;
                let bit7_is_set = value & NEGATIVE_BIT_MASK != 0;
                let new_value: u8 = value.wrapping_shl(1) | carry_bit_mask;
                self.flag_toggle(CpuFlags::Carry, bit7_is_set);
                self.flag_toggle(CpuFlags::Zero, new_value == 0);
                self.flag_toggle(CpuFlags::Negative, new_value & NEGATIVE_BIT_MASK != 0);
                bus.write(w, new_value)?;
            }
            _ => return Err(CpuError::InvalidAddressingMode),
        }
        Ok(())
    }

    pub(super) fn ror(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let carry_bit_mask = (self.flag_check(CpuFlags::Carry) as u8) << 7;
        match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(_) => {
                let bit0_is_set = self.accumulator & 1 == 1;
                self.accumulator = self.accumulator.wrapping_shr(1) | carry_bit_mask;
                self.flag_toggle(CpuFlags::Carry, bit0_is_set);
                self.flag_toggle(CpuFlags::Zero, self.accumulator == 0);
                self.flag_toggle(
                    CpuFlags::Negative,
                    self.accumulator & NEGATIVE_BIT_MASK != 0,
                );
            }
            OpcodeOperand::Address(w) => {
                let value = bus.read(w)?;
                let bit0_is_set = value & 1 == 1;
                let new_value: u8 = value.wrapping_shr(1) | carry_bit_mask;
                self.flag_toggle(CpuFlags::Carry, bit0_is_set);
                self.flag_toggle(CpuFlags::Zero, new_value == 0);
                self.flag_toggle(CpuFlags::Negative, new_value & NEGATIVE_BIT_MASK != 0);
                bus.write(w, new_value)?;
            }
            _ => return Err(CpuError::InvalidAddressingMode),
        }
        Ok(())
    }
}
