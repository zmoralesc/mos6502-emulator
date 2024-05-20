use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn clc(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_set(CpuFlags::Carry, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(in crate::mos6502) fn cld(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_set(CpuFlags::Decimal, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(in crate::mos6502) fn cli(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_set(CpuFlags::NoInterrupts, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(in crate::mos6502) fn clv(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_set(CpuFlags::Overflow, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(in crate::mos6502) fn sec(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_set(CpuFlags::Carry, true);
        self.increment_cycles(2);
        Ok(())
    }

    pub(in crate::mos6502) fn sed(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_set(CpuFlags::Decimal, true);
        self.increment_cycles(2);
        Ok(())
    }

    pub(in crate::mos6502) fn sei(&mut self, _: &mut T, _: AddressingMode) -> Result<(), CpuError> {
        self.flag_set(CpuFlags::NoInterrupts, true);
        self.increment_cycles(2);
        Ok(())
    }
}
