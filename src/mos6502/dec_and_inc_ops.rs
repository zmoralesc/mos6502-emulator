use super::*;

impl<T: Bus + Send + Sync> MOS6502<T> {
    pub(super) fn dec(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for DEC");
            }
        };
        let value = self.read_from_bus(addr)?.wrapping_sub(1);
        self.write_to_bus(addr, value)?;

        self.flag_toggle(FLAG_NEGATIVE, value & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, value == 0);

        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn dex(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.set_x_register(self.x_register.wrapping_sub(1));

        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.x_register == 0);

        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn dey(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.set_y_register(self.y_register.wrapping_sub(1));

        self.flag_toggle(FLAG_NEGATIVE, self.y_register & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.y_register == 0);

        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn inc(&mut self, address_mode: AddressingMode) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for INC");
            }
        };
        let value = self.read_from_bus(addr)?.wrapping_add(1);
        self.write_to_bus(addr, value)?;

        self.flag_toggle(FLAG_NEGATIVE, value & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, value == 0);

        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn inx(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.set_x_register(self.x_register.wrapping_add(1));

        self.flag_toggle(FLAG_NEGATIVE, self.x_register & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.x_register == 0);

        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn iny(&mut self, _: AddressingMode) -> Result<(), EmulationError> {
        self.set_y_register(self.y_register.wrapping_add(1));

        self.flag_toggle(FLAG_NEGATIVE, self.y_register & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.y_register == 0);

        self.increment_cycles(2);
        Ok(())
    }
}
