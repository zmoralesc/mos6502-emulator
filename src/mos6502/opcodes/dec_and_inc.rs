use crate::mos6502::*;

macro_rules! decrement_register {
    ($cpu:expr, $register:expr) => {
        $register = $register.wrapping_sub(1);

        $cpu.flag_toggle(CpuFlags::Negative, $register & NEGATIVE_BIT_MASK != 0);
        $cpu.flag_toggle(CpuFlags::Zero, $register == 0);

        $cpu.increment_cycles(2);
        return Ok(());
    };
}

macro_rules! increment_register {
    ($cpu:expr, $register:expr) => {
        $register = $register.wrapping_add(1);

        $cpu.flag_toggle(CpuFlags::Negative, $register & NEGATIVE_BIT_MASK != 0);
        $cpu.flag_toggle(CpuFlags::Zero, $register == 0);

        $cpu.increment_cycles(2);
        return Ok(());
    };
}

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn dec(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        let value = bus.read(addr)?.wrapping_sub(1);
        bus.write(addr, value)?;

        self.flag_toggle(CpuFlags::Negative, value & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(CpuFlags::Zero, value == 0);

        self.increment_cycles(3);
        Ok(())
    }

    pub(in crate::mos6502) fn dex(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        decrement_register!(self, self.x_register);
    }

    pub(in crate::mos6502) fn dey(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        decrement_register!(self, self.y_register);
    }

    pub(in crate::mos6502) fn inc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        let value = bus.read(addr)?.wrapping_add(1);
        bus.write(addr, value)?;

        self.flag_toggle(CpuFlags::Negative, value & NEGATIVE_BIT_MASK != 0);
        self.flag_toggle(CpuFlags::Zero, value == 0);

        self.increment_cycles(3);
        Ok(())
    }

    pub(in crate::mos6502) fn inx(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        increment_register!(self, self.x_register);
    }

    pub(in crate::mos6502) fn iny(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        increment_register!(self, self.y_register);
    }
}
