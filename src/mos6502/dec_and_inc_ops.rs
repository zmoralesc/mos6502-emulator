use super::*;

impl<T: Bus> MOS6502<T> {
    pub(super) fn dec(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(addr) => addr as u16,
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for DEC");
            }
        };
        let value = self.bus.read(addr).wrapping_sub(1);
        self.bus.write(addr, value);

        self.flag_toggle(FLAG_NEGATIVE, value & SIGN_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, value == 0);

        self.increment_cycles(3);
    }

    pub(super) fn dex(&mut self, _: AddressingMode) {
        self.x_register = self.x_register.wrapping_sub(1);

        self.flag_toggle(FLAG_NEGATIVE, self.x_register & SIGN_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.x_register == 0);

        self.increment_cycles(2);
    }

    pub(super) fn dey(&mut self, address_mode: AddressingMode) {
        self.y_register = self.y_register.wrapping_sub(1);

        self.flag_toggle(FLAG_NEGATIVE, self.y_register & SIGN_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.y_register == 0);

        self.increment_cycles(2);
    }

    pub(super) fn inc(&mut self, address_mode: AddressingMode) {
        let addr = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(addr) => addr as u16,
            OpcodeOperand::Address(addr) => addr,
            _ => {
                panic!("Invalid addressing mode for INC");
            }
        };
        let value = self.bus.read(addr).wrapping_add(1);
        self.bus.write(addr, value);

        self.flag_toggle(FLAG_NEGATIVE, value & SIGN_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, value == 0);

        self.increment_cycles(3);
    }

    pub(super) fn inx(&mut self, address_mode: AddressingMode) {
        self.x_register = self.x_register.wrapping_add(1);

        self.flag_toggle(FLAG_NEGATIVE, self.x_register & SIGN_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.x_register == 0);

        self.increment_cycles(2);
    }

    pub(super) fn iny(&mut self, address_mode: AddressingMode) {
        self.y_register = self.y_register.wrapping_add(1);

        self.flag_toggle(FLAG_NEGATIVE, self.y_register & SIGN_BIT_MASK != 0);
        self.flag_toggle(FLAG_ZERO, self.y_register == 0);

        self.increment_cycles(2);
    }
}
