#![allow(dead_code)]

use crate::memory::Bus;

const FLAG_NEGATIVE: u8 = 1 << 0;
const FLAG_OVERFLOW: u8 = 1 << 1;
const FLAG_BREAK: u8 = 1 << 3;
const FLAG_DECIMAL: u8 = 1 << 4;
const FLAG_NO_INTERRUPTS: u8 = 1 << 5;
const FLAG_ZERO: u8 = 1 << 6;
const FLAG_CARRY: u8 = 1 << 7;

type OpcodeFunction<'a, T> = fn(&mut MOS6502<'a, T>, AddressingMode);

enum OpcodeOperand<'a> {
    Byte(u8),
    Word(u16),
    Register(&'a u8),
    ProgramCounter(&'a u16),
}

#[derive(Clone, Copy)]
enum AddressingMode {
    Accumulator,
    Absolute,
    AbsoluteXIndex,
    AbsoluteYIndex,
    Immediate,
    Implied,
    Indirect,
    XIndexIndirect,
    IndirectYIndex,
    Relative,
    Zeropage,
    ZeropageXIndex,
    ZeropageYIndex,
}

pub struct MOS6502<'a, T: Bus> {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    sr: u8,
    pc: u16,
    cycles: u128,
    bus: &'a T,
    opcode_vec: Vec<(OpcodeFunction<'a, T>, AddressingMode)>,
}

impl<'a, T: Bus + Clone> MOS6502<'a, T> {
    pub fn new(bus: &'a mut T) -> MOS6502<'a, T> {
        MOS6502 {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0,
            sr: 0,
            cycles: 0,
            bus,
            opcode_vec: vec![(MOS6502::not_implemented, AddressingMode::Implied); 256],
        }
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    pub fn get_cycles(&self) -> u128 {
        self.cycles
    }

    pub fn run(&mut self) {
        let mut opc: u8;
        loop {
            opc = self.bus.bus_read(self.pc);
            let (opcode_func, address_mode) = self.opcode_vec[opc as usize];
            opcode_func(self, address_mode);
        }
    }

    pub fn run_for_cycles(&mut self, cycles: u128) {
        let mut opc: u8;
        while self.cycles < cycles {
            opc = self.bus.bus_read(self.pc);
            let (opcode_func, address_mode) = self.opcode_vec[opc as usize];
            opcode_func(self, address_mode);
        }
    }

    pub fn flag_check(&mut self, f: u8) -> bool {
        self.sr & f != 0
    }

    fn status_set(&mut self, f: u8, value: bool) {
        if value {
            self.sr |= f; // set flag
        } else {
            self.sr &= !f; // clear flag
        }
    }

    // load value into accumulator
    fn lda(&mut self, address_mode: AddressingMode) {
        self.a = match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Register(r) => *r,
            _ => {
                panic!("Invalid addressing mode for LDA");
            }
        };

        self.status_set(FLAG_ZERO, self.a == 0);
        self.status_set(FLAG_NEGATIVE, self.a & 0b10000000 != 0);

        self.pc += 1;
        self.cycles += 1;
    }

    // add to accumulator with carry
    fn adc(&mut self, address_mode: AddressingMode) {
        let a_oldvalue = self.a;
        self.a += match self.resolve_operand(address_mode) {
            OpcodeOperand::Byte(b) => self.bus.bus_read(b as u16),
            OpcodeOperand::Word(w) => self.bus.bus_read(w),
            _ => {
                panic!("Invalid addressing mode for ADC");
            }
        };
        self.status_set(FLAG_OVERFLOW, self.a < a_oldvalue);
    }

    fn not_implemented(&mut self, _: AddressingMode) {
        panic!("Opcode not implemented.\n");
    }

    // given some addressing mode, returns operand and increases cpu cycles appropriately
    fn resolve_operand(&mut self, address_mode: AddressingMode) -> OpcodeOperand {
        match address_mode {
            AddressingMode::Accumulator => {
                self.cycles += 1;
                OpcodeOperand::Register(&self.a)
            }
            AddressingMode::Absolute => {
                self.pc += 1;
                let low_byte = self.bus.bus_read(self.pc) as u16;
                self.pc += 1;
                let high_byte = self.bus.bus_read(self.pc) as u16;
                let addr: u16 = (high_byte << 8) | low_byte;
                OpcodeOperand::Byte(self.bus.bus_read(addr))
            }
            AddressingMode::AbsoluteXIndex => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::AbsoluteYIndex => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::Immediate => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::Implied => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::Indirect => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::XIndexIndirect => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::IndirectYIndex => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::Relative => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::Zeropage => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::ZeropageXIndex => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
            AddressingMode::ZeropageYIndex => {
                self.pc += 1;
                OpcodeOperand::Byte(0x00)
            }
        }
    }
}
