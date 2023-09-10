use super::*;

macro_rules! store_register_value {
    ($cpu:expr, $register:expr, $address_mode:ident, $bus:expr) => {
        let operand = $cpu.resolve_operand($bus, $address_mode)?;
        let addr = match operand {
            OpcodeOperand::Address(addr) => addr,
            _ => return Err(CpuError::InvalidAddressingMode),
        };
        $cpu.increment_cycles(1);
        $bus.write(addr, $register)?;
        return Ok(());
    };
}

macro_rules! load_value_to_register {
    ($cpu:expr, $register:expr, $address_mode:ident, $bus:expr) => {
        $cpu.increment_cycles(1);
        let operand = $cpu.resolve_operand($bus, $address_mode)?;
        $register = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => $bus.read(addr)?,
            _ => return Err(CpuError::InvalidAddressingMode),
        };

        $cpu.flag_toggle(CpuFlags::Zero, $register == 0);
        $cpu.flag_toggle(CpuFlags::Negative, $register & NEGATIVE_BIT_MASK != 0);
        return Ok(());
    };
}

macro_rules! transfer_register {
    ($cpu:expr, $source_register:expr, $target_register:expr) => {
        $cpu.increment_cycles(2);
        $target_register = $source_register;

        $cpu.flag_toggle(CpuFlags::Zero, $target_register == 0);
        $cpu.flag_toggle(
            CpuFlags::Negative,
            $target_register & NEGATIVE_BIT_MASK != 0,
        );
        return Ok(());
    };
}

impl<T: Bus> MOS6502<T> {
    // load value into accumulator
    pub(super) fn lda(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        load_value_to_register!(self, self.accumulator, address_mode, bus);
    }

    // load value into X register
    pub(super) fn ldx(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        load_value_to_register!(self, self.x_register, address_mode, bus);
    }

    // load value into Y register
    pub(super) fn ldy(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        load_value_to_register!(self, self.y_register, address_mode, bus);
    }

    // store accumulator in memory
    pub(super) fn sta(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        store_register_value!(self, self.accumulator, address_mode, bus);
    }

    // store X register in memory
    pub(super) fn stx(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        store_register_value!(self, self.x_register, address_mode, bus);
    }

    // store Y register in memory
    pub(super) fn sty(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<(), CpuError> {
        store_register_value!(self, self.y_register, address_mode, bus);
    }

    // transfer accumulator to X register
    pub(super) fn tax(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        transfer_register!(self, self.accumulator, self.x_register);
    }

    // transfer accumulator to Y register
    pub(super) fn tay(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        transfer_register!(self, self.accumulator, self.y_register);
    }

    // transfer stack pointer to X register
    pub(super) fn tsx(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        transfer_register!(self, self.stack_pointer, self.x_register);
    }

    // transfer X register to accumulator
    pub(super) fn txa(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        transfer_register!(self, self.x_register, self.accumulator);
    }

    // transfer X register to stack pointer
    pub(super) fn txs(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.increment_cycles(2);
        self.stack_pointer = self.x_register;

        Ok(())
    }

    // transfer Y register to accumulator
    pub(super) fn tya(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        transfer_register!(self, self.y_register, self.accumulator);
    }
}
