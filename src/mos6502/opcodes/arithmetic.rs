use crate::error::CpuError;

use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    #[inline(always)]
    fn add_to_accumulator_with_carry(&mut self, value: u8) -> Result<u32, CpuError> {
        let old_value = self.accumulator;

        let sign_bits_match: bool = (!(self.accumulator ^ value) & NEGATIVE_BIT_MASK) != 0;

        self.accumulator = self
            .accumulator
            .wrapping_add(value)
            .wrapping_add(self.flag_check(CpuFlags::Carry) as u8);

        let overflow: bool =
            sign_bits_match && ((self.accumulator ^ value) & NEGATIVE_BIT_MASK) != 0;

        let carry = self.accumulator < old_value || (self.accumulator == old_value && value != 0);

        self.flag_set(
            CpuFlags::Negative,
            self.accumulator & NEGATIVE_BIT_MASK != 0,
        );
        self.flag_set(CpuFlags::Zero, self.accumulator == 0);
        self.flag_set(CpuFlags::Carry, carry);
        self.flag_set(CpuFlags::Overflow, overflow);

        Ok(1)
    }

    // add to accumulator with carry
    pub(in crate::mos6502) fn adc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let value = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => bus.read(addr)?,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        Ok(self.add_to_accumulator_with_carry(value)?)
    }

    // subtract from accumulator with carry
    pub(in crate::mos6502) fn sbc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let value = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => bus.read(addr)?,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        Ok(self.add_to_accumulator_with_carry(!value)?)
    }
}
