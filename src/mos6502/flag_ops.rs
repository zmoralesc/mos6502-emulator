use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn clc(&mut self, _bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.flag_toggle(FLAG_CARRY, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn cld(&mut self, _bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.flag_toggle(FLAG_DECIMAL, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn cli(&mut self, _bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.flag_toggle(FLAG_NO_INTERRUPTS, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn clv(&mut self, _bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.flag_toggle(FLAG_OVERFLOW, false);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn sec(&mut self, _bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.flag_toggle(FLAG_CARRY, true);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn sed(&mut self, _bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.flag_toggle(FLAG_DECIMAL, true);
        self.increment_cycles(2);
        Ok(())
    }

    pub(super) fn sei(&mut self, _bus: &mut T, _: AddressingMode) -> Result<(), EmulationError> {
        self.flag_toggle(FLAG_NO_INTERRUPTS, true);
        self.increment_cycles(2);
        Ok(())
    }
}
