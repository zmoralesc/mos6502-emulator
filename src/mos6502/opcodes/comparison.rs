use crate::mos6502::*;
use std::cmp::Ordering;

impl<T: Bus> MOS6502<T> {
    #[inline(always)]
    fn compare_register(
        &mut self,
        register: u8,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let operand: u8 = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };

        self.increment_cycles(1);
        let result = register.wrapping_sub(operand);
        self.flag_toggle(CpuFlags::Negative, result & NEGATIVE_BIT_MASK != 0);

        match register.cmp(&operand) {
            Ordering::Less => {
                self.flag_toggle(CpuFlags::Zero, false);
                self.flag_toggle(CpuFlags::Carry, false);
            }
            Ordering::Equal => {
                self.flag_toggle(CpuFlags::Zero, true);
                self.flag_toggle(CpuFlags::Carry, true);
            }
            Ordering::Greater => {
                self.flag_toggle(CpuFlags::Zero, false);
                self.flag_toggle(CpuFlags::Carry, true);
            }
        }
        Ok(())
    }

    pub(in crate::mos6502) fn cmp(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        self.compare_register(self.accumulator, bus, address_mode)
    }

    pub(in crate::mos6502) fn cpx(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        self.compare_register(self.x_register, bus, address_mode)
    }

    pub(in crate::mos6502) fn cpy(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        self.compare_register(self.y_register, bus, address_mode)
    }
}
