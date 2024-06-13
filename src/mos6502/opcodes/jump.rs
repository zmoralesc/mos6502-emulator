use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn jmp(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let new_pc_value = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.set_program_counter(new_pc_value);
        Ok(cycles + 1)
    }

    pub(in crate::mos6502) fn jsr(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let return_address = self.program_counter + 1;

        let (return_address_lo, return_address_hi): (u8, u8) = return_address.to_le_bytes().into();

        self.push_to_stack(bus, return_address_hi)?;
        self.push_to_stack(bus, return_address_lo)?;

        let (cycles, operand) = self.resolve_operand(bus, address_mode)?;
        let new_pc_value = match operand {
            OpcodeOperand::Address(w) => w,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        self.set_program_counter(new_pc_value);
        Ok(cycles + 6)
    }

    pub(in crate::mos6502) fn rts(
        &mut self,
        bus: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        let return_address_lo = self.pop_from_stack(bus)?;
        let return_address_hi = self.pop_from_stack(bus)?;

        let return_address = u16::from_le_bytes([return_address_lo, return_address_hi]);
        self.set_program_counter(return_address.wrapping_add(1));
        Ok(6)
    }
}
