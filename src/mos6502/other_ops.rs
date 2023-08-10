use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn nop(&mut self, _: AddressingMode) {
        self.increment_cycles(2);
    }

    pub(super) fn bit(&mut self, address_mode: AddressingMode) {
        todo!()
    }
}
