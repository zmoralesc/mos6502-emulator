use super::*;
use std::cmp::Ordering;

macro_rules! compare_register {
    ($cpu:expr, $register:expr, $address_mode:ident, $bus:expr) => {
        let operand: u8 = match $cpu.resolve_operand($bus, $address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(w) => $bus.read(w)?,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };

        $cpu.increment_cycles(1);
        let result = $register.wrapping_sub(operand);
        $cpu.flag_toggle(FLAG_NEGATIVE, result & NEGATIVE_BIT_MASK != 0);

        match $register.cmp(&operand) {
            Ordering::Less => {
                $cpu.flag_toggle(FLAG_ZERO, false);
                $cpu.flag_toggle(FLAG_CARRY, false);
            }
            Ordering::Equal => {
                $cpu.flag_toggle(FLAG_ZERO, true);
                $cpu.flag_toggle(FLAG_CARRY, true);
            }
            Ordering::Greater => {
                $cpu.flag_toggle(FLAG_ZERO, false);
                $cpu.flag_toggle(FLAG_CARRY, true);
            }
        }
        return Ok(());
    };
}

impl<T: Bus> MOS6502<T> {
    pub(super) fn cmp(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        compare_register!(self, self.accumulator, address_mode, bus);
    }

    pub(super) fn cpx(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        compare_register!(self, self.x_register, address_mode, bus);
    }

    pub(super) fn cpy(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        compare_register!(self, self.y_register, address_mode, bus);
    }
}
