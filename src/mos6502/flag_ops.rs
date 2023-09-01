use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn clc(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_toggle(CpuFlags::Carry, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn cld(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_toggle(CpuFlags::Decimal, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn cli(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_toggle(CpuFlags::NoInterrupts, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn clv(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_toggle(CpuFlags::Overflow, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn sec(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_toggle(CpuFlags::Carry, true);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn sed(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_toggle(CpuFlags::Decimal, true);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn sei(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_toggle(CpuFlags::NoInterrupts, true);
        self.increment_cycles(2);
        Ok(())
    }
}
