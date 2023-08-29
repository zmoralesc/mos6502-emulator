use super::*;
use std::cmp::Ordering;

impl<T: Bus> MOS6502<T> {
    #[inline(always)]
    fn compare_register(
        &mut self,
        register: u8,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let operand: u8 = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => bus.read(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };

        self.increment_cycles(1);
        let result = register.wrapping_sub(operand);
        self.flag_toggle(FLAG_NEGATIVE, result & NEGATIVE_BIT_MASK != 0);

        match register.cmp(&operand) {
            Ordering::Less => {
                self.flag_toggle(FLAG_ZERO, false);
                self.flag_toggle(FLAG_CARRY, false);
            }
            Ordering::Equal => {
                self.flag_toggle(FLAG_ZERO, true);
                self.flag_toggle(FLAG_CARRY, true);
            }
            Ordering::Greater => {
                self.flag_toggle(FLAG_ZERO, false);
                self.flag_toggle(FLAG_CARRY, true);
            }
        }
        Ok(())
    }

    pub(super) fn cmp(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        self.compare_register(self.accumulator, bus, address_mode)
    }

    pub(super) fn cpx(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        self.compare_register(self.x_register, bus, address_mode)
    }

    pub(super) fn cpy(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        self.compare_register(self.y_register, bus, address_mode)
    }
}
