use crate::mos6502::*;

macro_rules! decrement_register {
    ($cpu:expr, $register:expr) => {
        $register = $register.wrapping_sub(1);

        $cpu.flag_set(CpuFlags::Negative, $register & NEGATIVE_BIT_MASK != 0);
        $cpu.flag_set(CpuFlags::Zero, $register == 0);

        return Ok(2);
    };
}

macro_rules! increment_register {
    ($cpu:expr, $register:expr) => {
        $register = $register.wrapping_add(1);

        $cpu.flag_set(CpuFlags::Negative, $register & NEGATIVE_BIT_MASK != 0);
        $cpu.flag_set(CpuFlags::Zero, $register == 0);

        return Ok(2);
    };
}

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn dec(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            OpcodeOperand::AddressWithOverflow(addr, _) => addr,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        let value = bus.read(addr)?.wrapping_sub(1);
        bus.write(addr, value)?;

        self.flag_set(CpuFlags::Negative, value & NEGATIVE_BIT_MASK != 0);
        self.flag_set(CpuFlags::Zero, value == 0);

        Ok(0)
    }

    pub(in crate::mos6502) fn dex(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        decrement_register!(self, self.x_register);
    }

    pub(in crate::mos6502) fn dey(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        decrement_register!(self, self.y_register);
    }

    pub(in crate::mos6502) fn inc(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        let addr = match self.resolve_operand(bus, address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            OpcodeOperand::AddressWithOverflow(addr, _) => addr,
            _ => return Err(CpuError::InvalidAddressingMode(address_mode)),
        };
        let value = bus.read(addr)?.wrapping_add(1);
        bus.write(addr, value)?;

        self.flag_set(CpuFlags::Negative, value & NEGATIVE_BIT_MASK != 0);
        self.flag_set(CpuFlags::Zero, value == 0);

        Ok(0)
    }

    pub(in crate::mos6502) fn inx(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        increment_register!(self, self.x_register);
    }

    pub(in crate::mos6502) fn iny(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        increment_register!(self, self.y_register);
    }
}
