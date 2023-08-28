use crate::error::EmulationError;

use super::*;

#[inline(always)]
fn do_adc<T: Bus>(cpu: &mut MOS6502<T>, value: u8) -> Result<(), EmulationError> {
    let old_value = cpu.accumulator;
    cpu.increment_cycles(1);

    let sign_bits_match: bool = (!(cpu.accumulator ^ value) & NEGATIVE_BIT_MASK) != 0;

    cpu.accumulator = cpu
        .accumulator
        .wrapping_add(value)
        .wrapping_add(cpu.flag_check(FLAG_CARRY) as u8);

    let overflow: bool = sign_bits_match && ((cpu.accumulator ^ value) & NEGATIVE_BIT_MASK) != 0;

    let carry = cpu.accumulator < old_value;

    cpu.flag_toggle(FLAG_NEGATIVE, cpu.accumulator & NEGATIVE_BIT_MASK != 0);
    cpu.flag_toggle(FLAG_ZERO, cpu.accumulator == 0);
    cpu.flag_toggle(FLAG_CARRY, carry);
    cpu.flag_toggle(FLAG_OVERFLOW, overflow);

    Ok(())
}

impl<T: Bus> MOS6502<T> {
    // add to accumulator with carry
    pub(super) fn adc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let value = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => bus.read(addr)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        do_adc(self, value)
    }

    // subtract from accumulator with carry
    pub(super) fn sbc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let value = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => bus.read(addr)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        do_adc(self, !value)
    }
}
