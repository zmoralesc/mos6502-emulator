use crate::error::CpuError;

use super::*;

impl<T: Bus> MOS6502<T> {
    #[inline(always)]
    fn add_to_accumulator_with_carry(&mut self, value: u8) -> Result<(), CpuError> {
        let old_value = self.accumulator;
        self.increment_cycles(1);

        let sign_bits_match: bool = (!(self.accumulator ^ value) & NEGATIVE_BIT_MASK) != 0;

        self.accumulator = self
            .accumulator
            .wrapping_add(value)
            .wrapping_add(self.flag_check(FLAG_CARRY) as u8);

        let overflow: bool =
            sign_bits_match && ((self.accumulator ^ value) & NEGATIVE_BIT_MASK) != 0;

        let carry = self.accumulator < old_value || (self.accumulator == old_value && value != 0);

        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_CARRY, carry);
        self.flag_toggle(FLAG_OVERFLOW, overflow);

        Ok(())
    }

    // add to accumulator with carry
    pub(super) fn adc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let value = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => bus.read(addr)?,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        self.add_to_accumulator_with_carry(value)
    }

    // subtract from accumulator with carry
    pub(super) fn sbc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let value = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => bus.read(addr)?,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        self.add_to_accumulator_with_carry(!value)
    }
}
