use crate::mos6502::*;

macro_rules! store_register_value {
    ($cpu:expr, $register:expr, $address_mode:ident, $bus:expr) => {
        let addr = match $cpu.resolve_operand($bus, $address_mode)? {
            OpcodeOperand::Address(addr) => addr,
            _ => return Err(CpuError::InvalidAddressingMode($address_mode)),
        };
        $bus.write(addr, $register)?;
        return Ok(0);
    };
}

macro_rules! load_value_to_register {
    ($cpu:expr, $register:expr, $address_mode:ident, $bus:expr) => {
        let mut extra_cycles = 0;
        $register = match $cpu.resolve_operand($bus, $address_mode)? {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => $bus.read(addr)?,
            OpcodeOperand::AddressWithOverflow(addr, overflow) => {
                extra_cycles += overflow as u32;
                $bus.read(addr)?
            }
            _ => return Err(CpuError::InvalidAddressingMode($address_mode)),
        };

        $cpu.flag_set(CpuFlags::Zero, $register == 0);
        $cpu.flag_set(CpuFlags::Negative, $register & NEGATIVE_BIT_MASK != 0);
        return Ok(extra_cycles);
    };
}

macro_rules! transfer_register {
    ($cpu:expr, $source_register:expr, $target_register:expr) => {
        $target_register = $source_register;

        $cpu.flag_set(CpuFlags::Zero, $target_register == 0);
        $cpu.flag_set(
            CpuFlags::Negative,
            $target_register & NEGATIVE_BIT_MASK != 0,
        );
        return Ok(2);
    };
}

impl<T: Bus> MOS6502<T> {
    // load value into accumulator
    pub(in crate::mos6502) fn lda(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        load_value_to_register!(self, self.accumulator, address_mode, bus);
    }

    // load value into X register
    pub(in crate::mos6502) fn ldx(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        load_value_to_register!(self, self.x_register, address_mode, bus);
    }

    // load value into Y register
    pub(in crate::mos6502) fn ldy(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        load_value_to_register!(self, self.y_register, address_mode, bus);
    }

    // store accumulator in memory
    pub(in crate::mos6502) fn sta(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        store_register_value!(self, self.accumulator, address_mode, bus);
    }

    // store X register in memory
    pub(in crate::mos6502) fn stx(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        store_register_value!(self, self.x_register, address_mode, bus);
    }

    // store Y register in memory
    pub(in crate::mos6502) fn sty(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<u32, CpuError> {
        store_register_value!(self, self.y_register, address_mode, bus);
    }

    // transfer accumulator to X register
    pub(in crate::mos6502) fn tax(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        transfer_register!(self, self.accumulator, self.x_register);
    }

    // transfer accumulator to Y register
    pub(in crate::mos6502) fn tay(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        transfer_register!(self, self.accumulator, self.y_register);
    }

    // transfer stack pointer to X register
    pub(in crate::mos6502) fn tsx(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        transfer_register!(self, self.stack_pointer, self.x_register);
    }

    // transfer X register to accumulator
    pub(in crate::mos6502) fn txa(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        transfer_register!(self, self.x_register, self.accumulator);
    }

    // transfer X register to stack pointer
    pub(in crate::mos6502) fn txs(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.stack_pointer = self.x_register;
        Ok(2)
    }

    // transfer Y register to accumulator
    pub(in crate::mos6502) fn tya(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        transfer_register!(self, self.y_register, self.accumulator);
    }
}
