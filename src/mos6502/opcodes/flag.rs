use crate::mos6502::*;

impl<T: Bus> MOS6502<T> {
    pub(in crate::mos6502) fn clc(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.flag_set(CpuFlags::Carry, false);
        Ok(2)
    }

    pub(in crate::mos6502) fn cld(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.flag_set(CpuFlags::Decimal, false);
        Ok(2)
    }

    pub(in crate::mos6502) fn cli(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.flag_set(CpuFlags::NoInterrupts, false);
        Ok(2)
    }

    pub(in crate::mos6502) fn clv(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.flag_set(CpuFlags::Overflow, false);
        Ok(2)
    }

    pub(in crate::mos6502) fn sec(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.flag_set(CpuFlags::Carry, true);
        Ok(2)
    }

    pub(in crate::mos6502) fn sed(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.flag_set(CpuFlags::Decimal, true);
        Ok(2)
    }

    pub(in crate::mos6502) fn sei(
        &mut self,
        _: &mut T,
        _: AddressingMode,
    ) -> Result<u32, CpuError> {
        self.flag_set(CpuFlags::NoInterrupts, true);
        Ok(2)
    }
}
