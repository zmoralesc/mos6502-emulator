use std::ops::{Shl, Shr};

use super::*;

impl<T: Bus + Send + Sync> MOS6502<T> {
    pub(super) fn asl(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(_) => {
                self.flag_toggle(FLAG_CARRY, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.accumulator <<= 1;
                self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
            }
            OpcodeOperand::Address(w) => {
                let value = self.read_from_bus(w)?;
                self.flag_toggle(FLAG_CARRY, value & 0b00000001 != 0);
                let new_value: u8 = value.shl(1);
                self.flag_toggle(FLAG_NEGATIVE, false);
                self.flag_toggle(FLAG_ZERO, new_value == 0);
                self.write_to_bus(w, new_value)?;
            }
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.increment_program_counter(1);
        Ok(())
    }

    pub(super) fn lsr(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(_) => {
                self.accumulator >>= 1;
                self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
            }
            OpcodeOperand::Address(w) => {
                let value = self.read_from_bus(w)?;
                let new_value: u8 = value.shr(1);
                self.flag_toggle(FLAG_NEGATIVE, false);
                self.flag_toggle(FLAG_ZERO, new_value == 0);
                self.write_to_bus(w, value)?;
            }
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.flag_toggle(FLAG_CARRY, false);
        self.increment_program_counter(1);
        Ok(())
    }

    pub(super) fn rol(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let carry_bit_mask = self.flag_check(FLAG_CARRY) as u8;
        match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(_) => {
                let bit7_is_set = self.accumulator & NEGATIVE_BIT_MASK != 0;
                self.set_accumulator(self.accumulator.shl(1) | carry_bit_mask);
                self.flag_toggle(FLAG_CARRY, bit7_is_set);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
                self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
            }
            OpcodeOperand::Address(w) => {
                let value = self.read_from_bus(w)?;
                let bit7_is_set = value & NEGATIVE_BIT_MASK != 0;
                let new_value: u8 = value.shl(1) | carry_bit_mask;
                self.flag_toggle(FLAG_CARRY, bit7_is_set);
                self.flag_toggle(FLAG_ZERO, new_value == 0);
                self.flag_toggle(FLAG_NEGATIVE, new_value & NEGATIVE_BIT_MASK != 0);
                self.write_to_bus(w, value)?;
            }
            _ => return Err(EmulationError::InvalidAddressingMode),
        }
        self.increment_program_counter(1);
        Ok(())
    }

    pub(super) fn ror(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let carry_bit_mask = (self.flag_check(FLAG_CARRY) as u8) << 7;
        match self.resolve_operand(address_mode)? {
            OpcodeOperand::Byte(_) => {
                let bit0_is_set = self.accumulator & 1 == 1;
                self.set_accumulator(self.accumulator.shr(1) | carry_bit_mask);
                self.flag_toggle(FLAG_CARRY, bit0_is_set);
                self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
                self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
            }
            OpcodeOperand::Address(w) => {
                let value = self.read_from_bus(w)?;
                let bit0_is_set = value & 1 == 1;
                let new_value: u8 = value.shr(1) | carry_bit_mask;
                self.flag_toggle(FLAG_CARRY, bit0_is_set);
                self.flag_toggle(FLAG_ZERO, new_value == 0);
                self.flag_toggle(FLAG_NEGATIVE, new_value & NEGATIVE_BIT_MASK != 0);
                self.write_to_bus(w, value)?;
            }
            _ => return Err(EmulationError::InvalidAddressingMode),
        }
        self.increment_program_counter(1);
        Ok(())
    }
}
