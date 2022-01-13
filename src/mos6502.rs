#![allow(dead_code)]

pub trait Bus {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn get_size(&self) -> usize;
}

const FLAG_NEGATIVE: u8 = 1 << 0;
const FLAG_OVERFLOW: u8 = 1 << 1;
const FLAG_BREAK: u8 = 1 << 3;
const FLAG_DECIMAL: u8 = 1 << 4;
const FLAG_NO_INTERRUPTS: u8 = 1 << 5;
const FLAG_ZERO: u8 = 1 << 6;
const FLAG_CARRY: u8 = 1 << 7;

type OpcodeFunction<'a, T> = fn(&mut MOS6502<'a, T>, AddressingMode);

enum OpcodeOperand {
    Byte(u8),
    Address(u16),
    None,
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
            opc = self.bus.read(self.pc);
            let (opcode_func, address_mode) = self.opcode_vec[opc as usize];
            opcode_func(self, address_mode);
        }
    }

    pub fn run_for_cycles(&mut self, cycles: u128) {
        let mut opc: u8;
        while self.cycles < cycles {
            opc = self.bus.read(self.pc);
            let (opcode_func, address_mode) = self.opcode_vec[opc as usize];
            opcode_func(self, address_mode);
        }
    }

    pub fn flag_check(&mut self, f: u8) -> bool {
        self.sr & f != 0
    }

    fn flag_toggle(&mut self, f: u8, value: bool) {
        if value {
            self.sr |= f; // set flag
        } else {
            self.sr &= !f; // clear flag
        }
    }

    // load value into accumulator
    fn lda(&mut self, address_mode: AddressingMode) {
        let operand = self.resolve_operand(address_mode);
        self.a = match operand {
            OpcodeOperand::Byte(b) => b,
            _ => {
                panic!("Invalid addressing mode for LDA");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.a == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.a & 0b10000000 != 0);

        self.pc += 1;
    }

    // add to accumulator with carry
    fn adc(&mut self, address_mode: AddressingMode) {
        let a_oldvalue = self.a;
        let operand = self.resolve_operand(address_mode);
        self.a += match operand {
            OpcodeOperand::Byte(b) => self.bus.read(b as u16),
            _ => {
                panic!("Invalid addressing mode for ADC");
            }
        };
        if self.flag_check(FLAG_CARRY) {
            self.a += 1;
        }
        self.flag_toggle(FLAG_OVERFLOW, self.a < a_oldvalue);
    }

    fn not_implemented(&mut self, _: AddressingMode) {
        panic!("Opcode not implemented.\n");
    }

    // given some addressing mode, returns operand and increases CPU cycles as appropriate
    fn resolve_operand(&mut self, address_mode: AddressingMode) -> OpcodeOperand {
        match address_mode {
            AddressingMode::Accumulator => {
                self.cycles += 1;
                OpcodeOperand::Byte(self.a)
            }
            AddressingMode::Absolute => {
                self.pc += 1;
                let low_byte: u8 = self.bus.read(self.pc);
                self.pc += 1;
                let high_byte: u8 = self.bus.read(self.pc);

                let addr = u16::from_le_bytes([low_byte, high_byte]);

                self.cycles += 2;
                OpcodeOperand::Address(addr)
            }
            AddressingMode::AbsoluteXIndex => {
                self.pc += 1;
                let low_byte: u8 = self.bus.read(self.pc);
                self.pc += 1;
                let high_byte: u8 = self.bus.read(self.pc);

                let mut addr = u16::from_le_bytes([low_byte, high_byte]) + self.x as u16;

                self.cycles += 2;
                if self.flag_check(FLAG_CARRY) {
                    let old_addr = addr;
                    addr += 1;
                    // add one more cycle if page boundaries were crossed
                    if old_addr & 0xFF00 != addr & 0xFF00 {
                        self.cycles += 1;
                    }
                }

                OpcodeOperand::Byte(self.bus.read(addr))
            }
            AddressingMode::AbsoluteYIndex => {
                self.pc += 1;
                let low_byte: u8 = self.bus.read(self.pc);
                self.pc += 1;
                let high_byte: u8 = self.bus.read(self.pc);

                let mut addr = u16::from_le_bytes([low_byte, high_byte]) + self.y as u16;

                self.cycles += 2;
                if self.flag_check(FLAG_CARRY) {
                    let old_addr = addr;
                    addr += 1;
                    // add one more cycle if page boundaries were crossed
                    if old_addr & 0xFF00 != addr & 0xFF00 {
                        self.cycles += 1;
                    }
                }

                OpcodeOperand::Byte(self.bus.read(addr))
            }
            AddressingMode::Immediate => {
                self.pc += 1;
                let byte: u8 = self.bus.read(self.pc);

                self.cycles += 1;
                OpcodeOperand::Byte(byte)
            }
            AddressingMode::Implied => OpcodeOperand::None,
            AddressingMode::Indirect => {
                self.pc += 1;
                let mut low_byte: u8 = self.bus.read(self.pc);
                self.pc += 1;
                let mut high_byte: u8 = self.bus.read(self.pc);

                let addr = u16::from_le_bytes([low_byte, high_byte]);

                low_byte = self.bus.read(addr);
                high_byte = self.bus.read(addr + 1);

                self.cycles += 2;
                OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]))
            }
            AddressingMode::XIndexIndirect => {
                self.pc += 1;
                let mut zp_addr: u16 = self.bus.read(self.pc) as u16;

                zp_addr += self.x as u16;

                let low_byte = self.bus.read(zp_addr);
                let high_byte = self.bus.read(zp_addr + 1);

                self.cycles += 6;
                OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]))
            }
            AddressingMode::IndirectYIndex => {
                self.pc += 1;
                let zp_addr: u16 = self.bus.read(self.pc) as u16;

                let low_byte = self.bus.read(zp_addr);
                let high_byte = self.bus.read(zp_addr + 1);

                self.cycles += 6;
                OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]) + self.y as u16)
            }
            AddressingMode::Relative => {
                self.pc += 1;
                let offset = self.bus.read(self.pc) as i16;

                let addr: u16 = if offset < 0 {
                    self.pc - offset.abs() as u16
                } else {
                    self.pc + offset.abs() as u16
                };

                OpcodeOperand::Address(addr as u16)
            }
            AddressingMode::Zeropage => {
                self.pc += 1;
                self.cycles += 1;

                let zp_addr = self.bus.read(self.pc) as u16;
                OpcodeOperand::Address(zp_addr)
            }
            AddressingMode::ZeropageXIndex => {
                self.pc += 1;
                self.cycles += 1;

                let offset = self.x as u16;
                let zp_addr = self.bus.read(self.pc) as u16;
                let addr: u16 = zp_addr + offset;

                OpcodeOperand::Address(addr)
            }
            AddressingMode::ZeropageYIndex => {
                self.pc += 1;
                self.cycles += 1;

                let offset = self.y as u16;
                let zp_addr = self.bus.read(self.pc) as u16;
                let addr: u16 = zp_addr + offset;

                OpcodeOperand::Address(addr)
            }
        }
    }
}
