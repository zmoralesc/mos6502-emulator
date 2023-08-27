use crate::{pop_from_stack, push_to_stack};

use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn jmp(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let new_pc_value = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.set_program_counter(new_pc_value);
        self.increment_cycles(1);
        Ok(())
    }

    pub(super) fn jsr(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let return_address = self.program_counter + 1;

        let (return_address_lo, return_address_hi): (u8, u8) = return_address.to_le_bytes().into();

        push_to_stack!(self, bus, return_address_hi);
        push_to_stack!(self, bus, return_address_lo);

        let new_pc_value = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(w) => w,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        self.set_program_counter(new_pc_value);
        self.increment_cycles(6);
        Ok(())
    }

    pub(super) fn rts(&mut self, bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        let return_address_lo = pop_from_stack!(self, bus);
        let return_address_hi = pop_from_stack!(self, bus);

        let return_address = u16::from_le_bytes([return_address_lo, return_address_hi]);
        self.set_program_counter(return_address.wrapping_add(1));
        self.increment_cycles(6);
        Ok(())
    }
}
