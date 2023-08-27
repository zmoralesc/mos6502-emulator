use super::*;

macro_rules! decrement_register {
    ($cpu:expr, $register:expr) => {
        $register = $register.wrapping_sub(1);

        $cpu.flag_toggle(FLAG_NEGATIVE, $register & NEGATIVE_BIT_MASK != 0);
        $cpu.flag_toggle(FLAG_ZERO, $register == 0);

        $cpu.increment_cycles(2);
        return Ok(());
    };
}

macro_rules! increment_register {
    ($cpu:expr, $register:expr) => {
        $register = $register.wrapping_add(1);

        $cpu.flag_toggle(FLAG_NEGATIVE, $register & NEGATIVE_BIT_MASK != 0);
        $cpu.flag_toggle(FLAG_ZERO, $register == 0);

        $cpu.increment_cycles(2);
        return Ok(());
    };
}

impl<T: Bus> MOS6502<T> {
    pub(super) fn dec(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        let value = bus.read(addr)?.wrapping_sub(1);
        bus.write(addr, value)?;

        self.flag_toggle(FLAG_NEGATIVE, value & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, value == 0);

        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn dex(&mut self, _: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        decrement_register!(self, self.x_register);
    }

    pub(super) fn dey(&mut self, _: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        decrement_register!(self, self.y_register);
    }

    pub(super) fn inc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), EmulationError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            _ => return Err(EmulationError::InvalidAddressingMode),
        };
        let value = bus.read(addr)?.wrapping_add(1);
        bus.write(addr, value)?;

        self.flag_toggle(FLAG_NEGATIVE, value & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, value == 0);

        self.increment_cycles(3);
        Ok(())
    }

    pub(super) fn inx(&mut self, _: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        increment_register!(self, self.x_register);
    }

    pub(super) fn iny(&mut self, _: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        increment_register!(self, self.y_register);
    }
}
