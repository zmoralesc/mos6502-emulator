pub const FLAG_NEGATIVE: u8 = 1 << 0;
pub const FLAG_OVERFLOW: u8 = 1 << 1;
pub const FLAG_BREAK: u8 = 1 << 3;
pub const FLAG_DECIMAL: u8 = 1 << 4;
pub const FLAG_NO_INTERRUPTS: u8 = 1 << 5;
pub const FLAG_ZERO: u8 = 1 << 6;
pub const FLAG_CARRY: u8 = 1 << 7;

pub struct MOS6502 {
    a: u8,
    x: u8,
    y: u8,
    pc: u16,
    sp: u8,
    sr: u8,
}

impl MOS6502 {
    pub fn new() -> MOS6502 {
        MOS6502 {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            sr: 0,
        }
    }

    fn flag_set(&mut self, f: u8) -> () {
        self.sr |= f;
    }

    fn flag_clear(&mut self, f: u8) -> () {
        self.sr &= !f;
    }

    pub fn flag_check(&mut self, f: u8) -> bool {
        self.sr & f != 0
    }
}
