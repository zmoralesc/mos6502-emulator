pub trait Memory {
    fn read(&self, address: usize) -> u8;
    fn write(&mut self, address: usize, value: u8) -> ();
    fn get_size(&self) -> usize;
}

pub struct RAM {
    size: usize,
    buffer: Vec<u8>,
}

impl RAM {
    pub fn new(size: usize) -> RAM {
        RAM {
            size,
            buffer: vec![0; size],
        }
    }
}

impl Memory for RAM {
    fn read(&self, address: usize) -> u8 {
        self.buffer[address]
    }
    fn write(&mut self, address: usize, value: u8) {
        self.buffer[address] = value;
    }
    fn get_size(&self) -> usize {
        self.size
    }
}

pub const FLAG_NEGATIVE: u8 = 1 << 0;
pub const FLAG_OVERFLOW: u8 = 1 << 1;
pub const FLAG_BREAK: u8 = 1 << 3;
pub const FLAG_DECIMAL: u8 = 1 << 4;
pub const FLAG_NO_INTERRUPTS: u8 = 1 << 5;
pub const FLAG_ZERO: u8 = 1 << 6;
pub const FLAG_CARRY: u8 = 1 << 7;

pub struct MOS6502<'a, M: Memory> {
    a: u8,
    x: u8,
    y: u8,
    pc: usize,
    sp: u8,
    sr: u8,
    mem: &'a M,
}

impl<'a, M: Memory> MOS6502<'a, M> {
    pub fn new(mem: &'a mut M) -> MOS6502<'a, M> {
        MOS6502 {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            sr: 0,
            mem,
        }
    }

    pub fn run(&mut self, mem: &'a mut M) {
        loop {
            break;
        }
    }

    pub fn flag_check(&mut self, f: u8) -> bool {
        self.sr & f != 0
    }

    fn flag_set(&mut self, f: u8) {
        self.sr |= f;
    }

    fn flag_clear(&mut self, f: u8) {
        self.sr &= !f;
    }

    fn lda(&mut self) {
        self.pc += 1;
        self.a = self.mem.read(self.pc);
    }
}
