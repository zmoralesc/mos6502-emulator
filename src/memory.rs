#![allow(dead_code)]

use crate::mos6502::Bus;

#[derive(Clone)]
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

impl Bus for RAM {
    fn read(&self, address: u16) -> u8 {
        self.buffer[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.buffer[address as usize] = value;
    }

    fn get_size(&self) -> usize {
        self.size
    }
}
