use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn asl(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(_) => {
                self.flag_toggle(FLAG_CARRY, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.accumulator = self.accumulator.wrapping_shl(1);
                self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
            }
            OpcodeOperand::Address(w) => {
                let mut value = bus.read(w)?;
                self.flag_toggle(FLAG_CARRY, value & NEGATIVE_BIT_MASK != 0);
                value = value.wrapping_shl(1);
                self.flag_toggle(FLAG_NEGATIVE, value & NEGATIVE_BIT_MASK != 0);
                self.flag_toggle(FLAG_ZERO, value == 0);
                bus.write(w, value)?;
            }
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        Ok(())
    }

    pub(super) fn lsr(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(_) => {
                let bit0_is_set = self.accumulator & 1 != 0;
                self.accumulator = self.accumulator.wrapping_shr(1);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
                self.flag_toggle(FLAG_CARRY, bit0_is_set);
            }
            OpcodeOperand::Address(w) => {
                let mut value = bus.read(w)?;
                let bit0_is_set = value & 1 != 0;
                value = value.wrapping_shr(1);
                self.flag_toggle(FLAG_ZERO, value == 0);
                self.flag_toggle(FLAG_CARRY, bit0_is_set);
                bus.write(w, value)?;
            }
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.flag_toggle(FLAG_NEGATIVE, false);
        Ok(())
    }

    pub(super) fn rol(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let carry_bit_mask = self.flag_check(FLAG_CARRY) as u8;
        match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(_) => {
                let bit7_is_set = self.accumulator & NEGATIVE_BIT_MASK != 0;
                self.accumulator = self.accumulator.wrapping_shl(1) | carry_bit_mask;
                self.flag_toggle(FLAG_CARRY, bit7_is_set);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
                self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
            }
            OpcodeOperand::Address(w) => {
                let value = bus.read(w)?;
                let bit7_is_set = value & NEGATIVE_BIT_MASK != 0;
                let new_value: u8 = value.wrapping_shl(1) | carry_bit_mask;
                self.flag_toggle(FLAG_CARRY, bit7_is_set);
                self.flag_toggle(FLAG_ZERO, new_value == 0);
                self.flag_toggle(FLAG_NEGATIVE, new_value & NEGATIVE_BIT_MASK != 0);
                bus.write(w, new_value)?;
            }
            _ => return Err(EmulationError::InvalidAddressingMode),
        }
        Ok(())
    }

    pub(super) fn ror(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let carry_bit_mask = (self.flag_check(FLAG_CARRY) as u8) << 7;
        match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(_) => {
                let bit0_is_set = self.accumulator & 1 == 1;
                self.accumulator = self.accumulator.wrapping_shr(1) | carry_bit_mask;
                self.flag_toggle(FLAG_CARRY, bit0_is_set);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
                self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
            }
            OpcodeOperand::Address(w) => {
                let value = bus.read(w)?;
                let bit0_is_set = value & 1 == 1;
                let new_value: u8 = value.wrapping_shr(1) | carry_bit_mask;
                self.flag_toggle(FLAG_CARRY, bit0_is_set);
                self.flag_toggle(FLAG_ZERO, new_value == 0);
                self.flag_toggle(FLAG_NEGATIVE, new_value & NEGATIVE_BIT_MASK != 0);
                bus.write(w, new_value)?;
            }
            _ => return Err(EmulationError::InvalidAddressingMode),
        }
        Ok(())
    }
}
