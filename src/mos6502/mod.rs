#![allow(dead_code)]

mod arithmetic_ops;
mod dec_and_inc_ops;
mod stack_ops;
mod transfer_ops;
mod logical_ops;

pub trait Bus {
    /// Read byte from bus
    fn read(&self, address: u16) -> u8;
    /// Write byte to bus
    fn write(&mut self, address: u16, value: u8);
    /// Get bus size in bytes
    fn size(&self) -> usize;
}

pub const FLAG_NEGATIVE: u8 = 1 << 0;
pub const FLAG_OVERFLOW: u8 = 1 << 1;
pub const FLAG_BREAK: u8 = 1 << 3;
pub const FLAG_DECIMAL: u8 = 1 << 4;
pub const FLAG_NO_INTERRUPTS: u8 = 1 << 5;
pub const FLAG_ZERO: u8 = 1 << 6;
pub const FLAG_CARRY: u8 = 1 << 7;

const STACK_BASE: u16 = 0x0100;

const MAGNITUDE_BIT_MASK: u8 = 0b01111111;
const SIGN_BIT_MASK: u8 = 0b10000000;

type OpcodeFunction<T> = fn(&mut MOS6502<T>, AddressingMode);
type OpcodeFunctionArray<T> = [(OpcodeFunction<T>, AddressingMode); 256];

enum OpcodeOperand {
    Byte(u8),
    Word(u16),
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

pub struct MOS6502<T: Bus> {
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    status_register: u8,
    program_counter: u16,
    cycles: u128,
    bus: T,
    opcode_array: OpcodeFunctionArray<T>,
}

impl<T: Bus> MOS6502<T> {
    /// Create new instance of MOS6502
    pub fn new(bus: T) -> MOS6502<T> {
        let opcode_array: OpcodeFunctionArray<T> = [
            (MOS6502::not_implemented, AddressingMode::Implied), // 00
            (MOS6502::not_implemented, AddressingMode::Implied), // 01
            (MOS6502::not_implemented, AddressingMode::Implied), // 02
            (MOS6502::not_implemented, AddressingMode::Implied), // 03
            (MOS6502::not_implemented, AddressingMode::Implied), // 04
            (MOS6502::not_implemented, AddressingMode::Implied), // 05
            (MOS6502::not_implemented, AddressingMode::Implied), // 06
            (MOS6502::not_implemented, AddressingMode::Implied), // 07
            (MOS6502::php, AddressingMode::Implied),             // 08
            (MOS6502::not_implemented, AddressingMode::Implied), // 09
            (MOS6502::not_implemented, AddressingMode::Implied), // 0A
            (MOS6502::not_implemented, AddressingMode::Implied), // 0B
            (MOS6502::not_implemented, AddressingMode::Implied), // 0C
            (MOS6502::not_implemented, AddressingMode::Implied), // 0D
            (MOS6502::not_implemented, AddressingMode::Implied), // 0E
            (MOS6502::not_implemented, AddressingMode::Implied), // 0F
            (MOS6502::not_implemented, AddressingMode::Implied), // 10
            (MOS6502::not_implemented, AddressingMode::Implied), // 11
            (MOS6502::not_implemented, AddressingMode::Implied), // 12
            (MOS6502::not_implemented, AddressingMode::Implied), // 13
            (MOS6502::not_implemented, AddressingMode::Implied), // 14
            (MOS6502::not_implemented, AddressingMode::Implied), // 15
            (MOS6502::not_implemented, AddressingMode::Implied), // 16
            (MOS6502::not_implemented, AddressingMode::Implied), // 17
            (MOS6502::not_implemented, AddressingMode::Implied), // 18
            (MOS6502::not_implemented, AddressingMode::Implied), // 19
            (MOS6502::not_implemented, AddressingMode::Implied), // 1A
            (MOS6502::not_implemented, AddressingMode::Implied), // 1B
            (MOS6502::not_implemented, AddressingMode::Implied), // 1C
            (MOS6502::not_implemented, AddressingMode::Implied), // 1D
            (MOS6502::not_implemented, AddressingMode::Implied), // 1E
            (MOS6502::not_implemented, AddressingMode::Implied), // 1F
            (MOS6502::not_implemented, AddressingMode::Implied), // 20
            (MOS6502::not_implemented, AddressingMode::Implied), // 21
            (MOS6502::not_implemented, AddressingMode::Implied), // 22
            (MOS6502::not_implemented, AddressingMode::Implied), // 23
            (MOS6502::not_implemented, AddressingMode::Implied), // 24
            (MOS6502::not_implemented, AddressingMode::Implied), // 25
            (MOS6502::not_implemented, AddressingMode::Implied), // 26
            (MOS6502::not_implemented, AddressingMode::Implied), // 27
            (MOS6502::plp, AddressingMode::Implied),             // 28
            (MOS6502::not_implemented, AddressingMode::Implied), // 29
            (MOS6502::not_implemented, AddressingMode::Implied), // 2A
            (MOS6502::not_implemented, AddressingMode::Implied), // 2B
            (MOS6502::not_implemented, AddressingMode::Implied), // 2C
            (MOS6502::not_implemented, AddressingMode::Implied), // 2D
            (MOS6502::not_implemented, AddressingMode::Implied), // 2E
            (MOS6502::not_implemented, AddressingMode::Implied), // 2F
            (MOS6502::not_implemented, AddressingMode::Implied), // 30
            (MOS6502::not_implemented, AddressingMode::Implied), // 31
            (MOS6502::not_implemented, AddressingMode::Implied), // 32
            (MOS6502::not_implemented, AddressingMode::Implied), // 33
            (MOS6502::not_implemented, AddressingMode::Implied), // 34
            (MOS6502::not_implemented, AddressingMode::Implied), // 35
            (MOS6502::not_implemented, AddressingMode::Implied), // 36
            (MOS6502::not_implemented, AddressingMode::Implied), // 37
            (MOS6502::not_implemented, AddressingMode::Implied), // 38
            (MOS6502::not_implemented, AddressingMode::Implied), // 39
            (MOS6502::not_implemented, AddressingMode::Implied), // 3A
            (MOS6502::not_implemented, AddressingMode::Implied), // 3B
            (MOS6502::not_implemented, AddressingMode::Implied), // 3C
            (MOS6502::not_implemented, AddressingMode::Implied), // 3D
            (MOS6502::not_implemented, AddressingMode::Implied), // 3E
            (MOS6502::not_implemented, AddressingMode::Implied), // 3F
            (MOS6502::not_implemented, AddressingMode::Implied), // 40
            (MOS6502::not_implemented, AddressingMode::Implied), // 41
            (MOS6502::not_implemented, AddressingMode::Implied), // 42
            (MOS6502::not_implemented, AddressingMode::Implied), // 43
            (MOS6502::not_implemented, AddressingMode::Implied), // 44
            (MOS6502::not_implemented, AddressingMode::Implied), // 45
            (MOS6502::not_implemented, AddressingMode::Implied), // 46
            (MOS6502::not_implemented, AddressingMode::Implied), // 47
            (MOS6502::pha, AddressingMode::Implied),             // 48
            (MOS6502::not_implemented, AddressingMode::Implied), // 49
            (MOS6502::not_implemented, AddressingMode::Implied), // 4A
            (MOS6502::not_implemented, AddressingMode::Implied), // 4B
            (MOS6502::not_implemented, AddressingMode::Implied), // 4C
            (MOS6502::not_implemented, AddressingMode::Implied), // 4D
            (MOS6502::not_implemented, AddressingMode::Implied), // 4E
            (MOS6502::not_implemented, AddressingMode::Implied), // 4F
            (MOS6502::not_implemented, AddressingMode::Implied), // 50
            (MOS6502::not_implemented, AddressingMode::Implied), // 51
            (MOS6502::not_implemented, AddressingMode::Implied), // 52
            (MOS6502::not_implemented, AddressingMode::Implied), // 53
            (MOS6502::not_implemented, AddressingMode::Implied), // 54
            (MOS6502::not_implemented, AddressingMode::Implied), // 55
            (MOS6502::not_implemented, AddressingMode::Implied), // 56
            (MOS6502::not_implemented, AddressingMode::Implied), // 57
            (MOS6502::not_implemented, AddressingMode::Implied), // 58
            (MOS6502::not_implemented, AddressingMode::Implied), // 59
            (MOS6502::not_implemented, AddressingMode::Implied), // 5A
            (MOS6502::not_implemented, AddressingMode::Implied), // 5B
            (MOS6502::not_implemented, AddressingMode::Implied), // 5C
            (MOS6502::not_implemented, AddressingMode::Implied), // 5D
            (MOS6502::not_implemented, AddressingMode::Implied), // 5E
            (MOS6502::not_implemented, AddressingMode::Implied), // 5F
            (MOS6502::not_implemented, AddressingMode::Implied), // 60
            (MOS6502::adc, AddressingMode::XIndexIndirect),      // 61
            (MOS6502::not_implemented, AddressingMode::Implied), // 62
            (MOS6502::not_implemented, AddressingMode::Implied), // 63
            (MOS6502::not_implemented, AddressingMode::Implied), // 64
            (MOS6502::adc, AddressingMode::Zeropage),            // 65
            (MOS6502::not_implemented, AddressingMode::Implied), // 66
            (MOS6502::not_implemented, AddressingMode::Implied), // 67
            (MOS6502::pla, AddressingMode::Implied),             // 68
            (MOS6502::adc, AddressingMode::Immediate),           // 69
            (MOS6502::not_implemented, AddressingMode::Implied), // 6A
            (MOS6502::not_implemented, AddressingMode::Implied), // 6B
            (MOS6502::not_implemented, AddressingMode::Implied), // 6C
            (MOS6502::adc, AddressingMode::Absolute),            // 6D
            (MOS6502::not_implemented, AddressingMode::Implied), // 6E
            (MOS6502::not_implemented, AddressingMode::Implied), // 6F
            (MOS6502::not_implemented, AddressingMode::Implied), // 70
            (MOS6502::adc, AddressingMode::IndirectYIndex),      // 71
            (MOS6502::not_implemented, AddressingMode::Implied), // 72
            (MOS6502::not_implemented, AddressingMode::Implied), // 73
            (MOS6502::not_implemented, AddressingMode::Implied), // 74
            (MOS6502::adc, AddressingMode::ZeropageXIndex),      // 75
            (MOS6502::not_implemented, AddressingMode::Implied), // 76
            (MOS6502::not_implemented, AddressingMode::Implied), // 77
            (MOS6502::not_implemented, AddressingMode::Implied), // 78
            (MOS6502::adc, AddressingMode::AbsoluteYIndex),      // 79
            (MOS6502::not_implemented, AddressingMode::Implied), // 7A
            (MOS6502::not_implemented, AddressingMode::Implied), // 7B
            (MOS6502::not_implemented, AddressingMode::Implied), // 7C
            (MOS6502::adc, AddressingMode::AbsoluteXIndex),      // 7D
            (MOS6502::not_implemented, AddressingMode::Implied), // 7E
            (MOS6502::not_implemented, AddressingMode::Implied), // 7F
            (MOS6502::not_implemented, AddressingMode::Implied), // 80
            (MOS6502::not_implemented, AddressingMode::Implied), // 81
            (MOS6502::not_implemented, AddressingMode::Implied), // 82
            (MOS6502::not_implemented, AddressingMode::Implied), // 83
            (MOS6502::not_implemented, AddressingMode::Implied), // 84
            (MOS6502::not_implemented, AddressingMode::Implied), // 85
            (MOS6502::not_implemented, AddressingMode::Implied), // 86
            (MOS6502::not_implemented, AddressingMode::Implied), // 87
            (MOS6502::dey, AddressingMode::Implied),             // 88
            (MOS6502::not_implemented, AddressingMode::Implied), // 89
            (MOS6502::txa, AddressingMode::Implied),             // 8A
            (MOS6502::not_implemented, AddressingMode::Implied), // 8B
            (MOS6502::not_implemented, AddressingMode::Implied), // 8C
            (MOS6502::not_implemented, AddressingMode::Implied), // 8D
            (MOS6502::not_implemented, AddressingMode::Implied), // 8E
            (MOS6502::not_implemented, AddressingMode::Implied), // 8F
            (MOS6502::not_implemented, AddressingMode::Implied), // 90
            (MOS6502::not_implemented, AddressingMode::Implied), // 91
            (MOS6502::not_implemented, AddressingMode::Implied), // 92
            (MOS6502::not_implemented, AddressingMode::Implied), // 93
            (MOS6502::not_implemented, AddressingMode::Implied), // 94
            (MOS6502::not_implemented, AddressingMode::Implied), // 95
            (MOS6502::not_implemented, AddressingMode::Implied), // 96
            (MOS6502::not_implemented, AddressingMode::Implied), // 97
            (MOS6502::not_implemented, AddressingMode::Implied), // 98
            (MOS6502::not_implemented, AddressingMode::Implied), // 99
            (MOS6502::not_implemented, AddressingMode::Implied), // 9A
            (MOS6502::not_implemented, AddressingMode::Implied), // 9B
            (MOS6502::not_implemented, AddressingMode::Implied), // 9C
            (MOS6502::not_implemented, AddressingMode::Implied), // 9D
            (MOS6502::not_implemented, AddressingMode::Implied), // 9E
            (MOS6502::not_implemented, AddressingMode::Implied), // 9F
            (MOS6502::ldy, AddressingMode::Immediate),           // A0
            (MOS6502::lda, AddressingMode::XIndexIndirect),      // A1
            (MOS6502::ldx, AddressingMode::Immediate),           // A2
            (MOS6502::not_implemented, AddressingMode::Implied), // A3
            (MOS6502::ldy, AddressingMode::Zeropage),            // A4
            (MOS6502::lda, AddressingMode::Zeropage),            // A5
            (MOS6502::ldx, AddressingMode::Zeropage),            // A6
            (MOS6502::not_implemented, AddressingMode::Implied), // A7
            (MOS6502::not_implemented, AddressingMode::Implied), // A8
            (MOS6502::lda, AddressingMode::Immediate),           // A9
            (MOS6502::not_implemented, AddressingMode::Implied), // AA
            (MOS6502::not_implemented, AddressingMode::Implied), // AB
            (MOS6502::ldy, AddressingMode::Absolute),            // AC
            (MOS6502::lda, AddressingMode::Absolute),            // AD
            (MOS6502::ldx, AddressingMode::Absolute),            // AE
            (MOS6502::not_implemented, AddressingMode::Implied), // AF
            (MOS6502::not_implemented, AddressingMode::Implied), // B0
            (MOS6502::lda, AddressingMode::IndirectYIndex),      // B1
            (MOS6502::not_implemented, AddressingMode::Implied), // B2
            (MOS6502::not_implemented, AddressingMode::Implied), // B3
            (MOS6502::ldy, AddressingMode::ZeropageXIndex),      // B4
            (MOS6502::lda, AddressingMode::ZeropageXIndex),      // B5
            (MOS6502::ldx, AddressingMode::ZeropageYIndex),      // B6
            (MOS6502::not_implemented, AddressingMode::Implied), // B7
            (MOS6502::not_implemented, AddressingMode::Implied), // B8
            (MOS6502::lda, AddressingMode::AbsoluteYIndex),      // B9
            (MOS6502::not_implemented, AddressingMode::Implied), // BA
            (MOS6502::not_implemented, AddressingMode::Implied), // BB
            (MOS6502::ldy, AddressingMode::AbsoluteXIndex),      // BC
            (MOS6502::lda, AddressingMode::AbsoluteXIndex),      // BD
            (MOS6502::ldx, AddressingMode::AbsoluteYIndex),      // BE
            (MOS6502::not_implemented, AddressingMode::Implied), // BF
            (MOS6502::not_implemented, AddressingMode::Implied), // C0
            (MOS6502::not_implemented, AddressingMode::Implied), // C1
            (MOS6502::not_implemented, AddressingMode::Implied), // C2
            (MOS6502::not_implemented, AddressingMode::Implied), // C3
            (MOS6502::not_implemented, AddressingMode::Implied), // C4
            (MOS6502::not_implemented, AddressingMode::Implied), // C5
            (MOS6502::dec, AddressingMode::Zeropage),            // C6
            (MOS6502::not_implemented, AddressingMode::Implied), // C7
            (MOS6502::iny, AddressingMode::Implied),             // C8
            (MOS6502::not_implemented, AddressingMode::Implied), // C9
            (MOS6502::dex, AddressingMode::Implied),             // CA
            (MOS6502::not_implemented, AddressingMode::Implied), // CB
            (MOS6502::not_implemented, AddressingMode::Implied), // CC
            (MOS6502::not_implemented, AddressingMode::Implied), // CD
            (MOS6502::dec, AddressingMode::Absolute),            // CE
            (MOS6502::not_implemented, AddressingMode::Implied), // CF
            (MOS6502::not_implemented, AddressingMode::Implied), // D0
            (MOS6502::not_implemented, AddressingMode::Implied), // D1
            (MOS6502::not_implemented, AddressingMode::Implied), // D2
            (MOS6502::not_implemented, AddressingMode::Implied), // D3
            (MOS6502::not_implemented, AddressingMode::Implied), // D4
            (MOS6502::not_implemented, AddressingMode::Implied), // D5
            (MOS6502::dec, AddressingMode::ZeropageXIndex),      // D6
            (MOS6502::not_implemented, AddressingMode::Implied), // D7
            (MOS6502::not_implemented, AddressingMode::Implied), // D8
            (MOS6502::not_implemented, AddressingMode::Implied), // D9
            (MOS6502::not_implemented, AddressingMode::Implied), // DA
            (MOS6502::not_implemented, AddressingMode::Implied), // DB
            (MOS6502::not_implemented, AddressingMode::Implied), // DC
            (MOS6502::not_implemented, AddressingMode::Implied), // DD
            (MOS6502::dec, AddressingMode::AbsoluteXIndex),      // DE
            (MOS6502::not_implemented, AddressingMode::Implied), // DF
            (MOS6502::not_implemented, AddressingMode::Implied), // E0
            (MOS6502::sbc, AddressingMode::XIndexIndirect),      // E1
            (MOS6502::not_implemented, AddressingMode::Implied), // E2
            (MOS6502::not_implemented, AddressingMode::Implied), // E3
            (MOS6502::not_implemented, AddressingMode::Implied), // E4
            (MOS6502::sbc, AddressingMode::Zeropage),            // E5
            (MOS6502::inc, AddressingMode::Zeropage),            // E6
            (MOS6502::not_implemented, AddressingMode::Implied), // E7
            (MOS6502::inx, AddressingMode::Implied),             // E8
            (MOS6502::sbc, AddressingMode::Immediate),           // E9
            (MOS6502::not_implemented, AddressingMode::Implied), // EA
            (MOS6502::not_implemented, AddressingMode::Implied), // EB
            (MOS6502::not_implemented, AddressingMode::Implied), // EC
            (MOS6502::sbc, AddressingMode::Absolute),            // ED
            (MOS6502::inc, AddressingMode::Absolute),            // EE
            (MOS6502::not_implemented, AddressingMode::Implied), // EF
            (MOS6502::not_implemented, AddressingMode::Implied), // F0
            (MOS6502::sbc, AddressingMode::IndirectYIndex),      // F1
            (MOS6502::not_implemented, AddressingMode::Implied), // F2
            (MOS6502::not_implemented, AddressingMode::Implied), // F3
            (MOS6502::not_implemented, AddressingMode::Implied), // F4
            (MOS6502::sbc, AddressingMode::ZeropageXIndex),      // F5
            (MOS6502::inc, AddressingMode::ZeropageXIndex),      // F6
            (MOS6502::not_implemented, AddressingMode::Implied), // F7
            (MOS6502::not_implemented, AddressingMode::Implied), // F8
            (MOS6502::sbc, AddressingMode::AbsoluteYIndex),      // F9
            (MOS6502::not_implemented, AddressingMode::Implied), // FA
            (MOS6502::not_implemented, AddressingMode::Implied), // FB
            (MOS6502::not_implemented, AddressingMode::Implied), // FC
            (MOS6502::sbc, AddressingMode::AbsoluteXIndex),      // FD
            (MOS6502::inc, AddressingMode::AbsoluteXIndex),      // FE
            (MOS6502::not_implemented, AddressingMode::Implied), // FF
        ];
        MOS6502 {
            accumulator: u8::MIN,
            x_register: u8::MIN,
            y_register: u8::MIN,
            program_counter: u16::MIN,
            stack_pointer: u8::MAX,
            status_register: u8::MIN,
            cycles: u128::MIN,
            bus,
            opcode_array,
        }
    }

    fn not_implemented(&mut self, _: AddressingMode) {
        panic!("Opcode not implemented.")
    }

    /// Change value of program counter
    pub fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }

    /// Return number of elapsed CPU cycles
    pub fn cycles(&self) -> u128 {
        self.cycles
    }

    /// Start CPU
    pub fn run(&mut self) {
        let mut opc: u8;
        loop {
            opc = self.bus.read(self.program_counter);
            let (opcode_func, address_mode) = self.opcode_array[opc as usize];
            opcode_func(self, address_mode);
            self.increment_program_counter(1);
        }
    }

    /// Run CPU for a specific number of cycles
    pub fn run_for_cycles(&mut self, cycles: u128) {
        let mut opc: u8;
        while self.cycles < cycles {
            opc = self.bus.read(self.program_counter);
            let (opcode_func, address_mode) = self.opcode_array[opc as usize];
            opcode_func(self, address_mode);
            self.increment_program_counter(1);
        }
    }

    pub fn bus(&mut self) -> &mut impl Bus {
        &mut self.bus
    }

    /// Check if specified flag is set
    pub fn flag_check(&self, f: u8) -> bool {
        self.status_register & f != 0
    }

    pub fn accumulator(&self) -> u8 {
        self.accumulator
    }

    pub fn x_register(&self) -> u8 {
        self.x_register
    }

    pub fn y_register(&self) -> u8 {
        self.y_register
    }

    fn increment_program_counter(&mut self, n: u16) {
        self.program_counter = self.program_counter.wrapping_add(n);
    }

    fn increment_cycles(&mut self, n: u128) {
        self.cycles = self.cycles.wrapping_add(n);
    }

    /// Turn specified flag on/off
    fn flag_toggle(&mut self, f: u8, value: bool) {
        if value {
            self.status_register |= f; // set flag
        } else {
            self.status_register &= !f; // clear flag
        }
    }

    /// Given some addressing mode, returns operand and increases CPU cycles as appropriate
    fn resolve_operand(&mut self, address_mode: AddressingMode) -> OpcodeOperand {
        match address_mode {
            AddressingMode::Accumulator => {
                self.increment_cycles(1);
                OpcodeOperand::Byte(self.accumulator)
            }
            AddressingMode::Absolute => {
                self.increment_program_counter(1);
                let low_byte: u8 = self.bus.read(self.program_counter);
                self.increment_program_counter(1);
                let high_byte: u8 = self.bus.read(self.program_counter);

                let addr = u16::from_le_bytes([low_byte, high_byte]);

                self.increment_cycles(2);
                OpcodeOperand::Word(addr)
            }
            AddressingMode::AbsoluteXIndex => {
                self.increment_program_counter(1);
                let low_byte: u8 = self.bus.read(self.program_counter);
                self.increment_program_counter(1);
                let high_byte: u8 = self.bus.read(self.program_counter);

                let mut addr =
                    u16::from_le_bytes([low_byte, high_byte]).wrapping_add(self.x_register as u16);

                self.increment_cycles(2);
                if self.flag_check(FLAG_CARRY) {
                    let old_addr = addr;
                    addr = addr.wrapping_add(1);
                    // add one more cycle if page boundaries were crossed
                    self.increment_cycles((old_addr & 0xFF00 != addr & 0xFF00) as u128);
                }

                self.increment_cycles(1);
                OpcodeOperand::Byte(self.bus.read(addr))
            }
            AddressingMode::AbsoluteYIndex => {
                self.increment_program_counter(1);
                let low_byte: u8 = self.bus.read(self.program_counter);
                self.increment_program_counter(1);
                let high_byte: u8 = self.bus.read(self.program_counter);

                let mut addr =
                    u16::from_le_bytes([low_byte, high_byte]).wrapping_add(self.y_register as u16);

                self.increment_cycles(2);
                if self.flag_check(FLAG_CARRY) {
                    let old_addr = addr;
                    addr = addr.wrapping_add(1);
                    // add one more cycle if page boundaries were crossed
                    self.increment_cycles((old_addr & 0xFF00 != addr & 0xFF00) as u128);
                }

                self.increment_cycles(1);
                OpcodeOperand::Byte(self.bus.read(addr))
            }
            AddressingMode::Immediate => {
                self.increment_program_counter(1);
                let byte: u8 = self.bus.read(self.program_counter);

                self.increment_cycles(1);
                OpcodeOperand::Byte(byte)
            }
            AddressingMode::Implied => OpcodeOperand::None,
            AddressingMode::Indirect => {
                self.increment_program_counter(1);
                let mut low_byte: u8 = self.bus.read(self.program_counter);
                self.increment_program_counter(1);
                let mut high_byte: u8 = self.bus.read(self.program_counter);

                let addr = u16::from_le_bytes([low_byte, high_byte]);

                low_byte = self.bus.read(addr);
                high_byte = self.bus.read(addr.wrapping_add(1));

                self.increment_cycles(2);
                OpcodeOperand::Word(u16::from_le_bytes([low_byte, high_byte]))
            }
            AddressingMode::XIndexIndirect => {
                self.increment_program_counter(1);
                let mut zp_addr: u8 = self.bus.read(self.program_counter);

                zp_addr = zp_addr.wrapping_add(self.x_register);

                let low_byte = self.bus.read(zp_addr as u16);
                let high_byte = self.bus.read(zp_addr.wrapping_add(1) as u16);

                self.increment_cycles(6);
                OpcodeOperand::Word(u16::from_le_bytes([low_byte, high_byte]))
            }
            AddressingMode::IndirectYIndex => {
                self.increment_program_counter(1);
                let zp_addr = self.bus.read(self.program_counter);

                let low_byte = self.bus.read(zp_addr as u16);
                let high_byte = self.bus.read(zp_addr.wrapping_add(1) as u16);

                self.increment_cycles(6);
                OpcodeOperand::Word(
                    u16::from_le_bytes([low_byte, high_byte]).wrapping_add(self.y_register as u16),
                )
            }
            AddressingMode::Relative => {
                self.increment_program_counter(1);
                let offset_byte = self.bus.read(self.program_counter);

                let offset_magnitude = offset_byte & MAGNITUDE_BIT_MASK;
                let is_negative = offset_byte & SIGN_BIT_MASK != 0;

                let addr: u16 = if !is_negative {
                    self.program_counter.wrapping_add(offset_magnitude as u16)
                } else {
                    self.program_counter
                        .wrapping_add((offset_magnitude as u16).wrapping_neg())
                };

                OpcodeOperand::Word(addr)
            }
            AddressingMode::Zeropage => {
                self.increment_program_counter(1);
                self.increment_cycles(1);

                let zp_addr = self.bus.read(self.program_counter);
                self.increment_cycles(1);
                OpcodeOperand::Word(zp_addr as u16)
            }
            AddressingMode::ZeropageXIndex => {
                self.increment_program_counter(1);
                self.increment_cycles(1);

                let offset = self.x_register;
                let zp_addr = self.bus.read(self.program_counter);
                self.increment_cycles(1);

                let addr = zp_addr.wrapping_add(offset);
                self.increment_cycles(1);

                OpcodeOperand::Word(addr as u16)
            }
            AddressingMode::ZeropageYIndex => {
                self.increment_program_counter(1);
                self.increment_cycles(1);

                let offset = self.y_register;
                let zp_addr = self.bus.read(self.program_counter);
                self.increment_cycles(1);

                let addr = zp_addr.wrapping_add(offset);
                self.increment_cycles(1);

                OpcodeOperand::Word(addr as u16)
            }
        }
    }
}
