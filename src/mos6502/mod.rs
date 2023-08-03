#![allow(dead_code)]

pub trait Bus {
    /// Read byte from bus
    fn read(&self, address: u16) -> u8;
    /// Write byte to bus
    fn write(&mut self, address: u16, value: u8);
    /// Get bus size in bytes
    fn get_size(&self) -> usize;
}

const FLAG_NEGATIVE: u8 = 1 << 0;
const FLAG_OVERFLOW: u8 = 1 << 1;
const FLAG_BREAK: u8 = 1 << 3;
const FLAG_DECIMAL: u8 = 1 << 4;
const FLAG_NO_INTERRUPTS: u8 = 1 << 5;
const FLAG_ZERO: u8 = 1 << 6;
const FLAG_CARRY: u8 = 1 << 7;

type OpcodeFunction<T> = fn(&mut MOS6502<T>, AddressingMode);

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

pub struct MOS6502<T: Bus> {
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    status_register: u8,
    program_counter: u16,
    cycles: u128,
    bus: T,
    opcode_vec: Vec<(OpcodeFunction<T>, AddressingMode)>,
}

impl<T: Bus> MOS6502<T> {
    /// Create new instance of MOS6502
    pub fn new(bus: T) -> MOS6502<T> {
        MOS6502 {
            accumulator: 0x00_u8,
            x_register: 0x00_u8,
            y_register: 0x00_u8,
            program_counter: 0x00_u16,
            stack_pointer: 0x00_u8,
            status_register: 0x00_u8,
            cycles: 0x00_u128,
            bus,
            opcode_vec: vec![
                (MOS6502::not_implemented, AddressingMode::Implied), // 00
                (MOS6502::not_implemented, AddressingMode::Implied), // 01
                (MOS6502::not_implemented, AddressingMode::Implied), // 02
                (MOS6502::not_implemented, AddressingMode::Implied), // 03
                (MOS6502::not_implemented, AddressingMode::Implied), // 04
                (MOS6502::not_implemented, AddressingMode::Implied), // 05
                (MOS6502::not_implemented, AddressingMode::Implied), // 06
                (MOS6502::not_implemented, AddressingMode::Implied), // 07
                (MOS6502::not_implemented, AddressingMode::Implied), // 08
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
                (MOS6502::not_implemented, AddressingMode::Implied), // 28
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
                (MOS6502::not_implemented, AddressingMode::Implied), // 48
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
                (MOS6502::not_implemented, AddressingMode::Implied), // 61
                (MOS6502::not_implemented, AddressingMode::Implied), // 62
                (MOS6502::not_implemented, AddressingMode::Implied), // 63
                (MOS6502::not_implemented, AddressingMode::Implied), // 64
                (MOS6502::not_implemented, AddressingMode::Implied), // 65
                (MOS6502::not_implemented, AddressingMode::Implied), // 66
                (MOS6502::not_implemented, AddressingMode::Implied), // 67
                (MOS6502::not_implemented, AddressingMode::Implied), // 68
                (MOS6502::not_implemented, AddressingMode::Implied), // 69
                (MOS6502::not_implemented, AddressingMode::Implied), // 6A
                (MOS6502::not_implemented, AddressingMode::Implied), // 6B
                (MOS6502::not_implemented, AddressingMode::Implied), // 6C
                (MOS6502::not_implemented, AddressingMode::Implied), // 6D
                (MOS6502::not_implemented, AddressingMode::Implied), // 6E
                (MOS6502::not_implemented, AddressingMode::Implied), // 6F
                (MOS6502::not_implemented, AddressingMode::Implied), // 70
                (MOS6502::not_implemented, AddressingMode::Implied), // 71
                (MOS6502::not_implemented, AddressingMode::Implied), // 72
                (MOS6502::not_implemented, AddressingMode::Implied), // 73
                (MOS6502::not_implemented, AddressingMode::Implied), // 74
                (MOS6502::not_implemented, AddressingMode::Implied), // 75
                (MOS6502::not_implemented, AddressingMode::Implied), // 76
                (MOS6502::not_implemented, AddressingMode::Implied), // 77
                (MOS6502::not_implemented, AddressingMode::Implied), // 78
                (MOS6502::not_implemented, AddressingMode::Implied), // 79
                (MOS6502::not_implemented, AddressingMode::Implied), // 7A
                (MOS6502::not_implemented, AddressingMode::Implied), // 7B
                (MOS6502::not_implemented, AddressingMode::Implied), // 7C
                (MOS6502::not_implemented, AddressingMode::Implied), // 7D
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
                (MOS6502::not_implemented, AddressingMode::Implied), // 88
                (MOS6502::not_implemented, AddressingMode::Implied), // 89
                (MOS6502::not_implemented, AddressingMode::Implied), // 8A
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
                (MOS6502::not_implemented, AddressingMode::Implied), // C6
                (MOS6502::not_implemented, AddressingMode::Implied), // C7
                (MOS6502::not_implemented, AddressingMode::Implied), // C8
                (MOS6502::not_implemented, AddressingMode::Implied), // C9
                (MOS6502::not_implemented, AddressingMode::Implied), // CA
                (MOS6502::not_implemented, AddressingMode::Implied), // CB
                (MOS6502::not_implemented, AddressingMode::Implied), // CC
                (MOS6502::not_implemented, AddressingMode::Implied), // CD
                (MOS6502::not_implemented, AddressingMode::Implied), // CE
                (MOS6502::not_implemented, AddressingMode::Implied), // CF
                (MOS6502::not_implemented, AddressingMode::Implied), // D0
                (MOS6502::not_implemented, AddressingMode::Implied), // D1
                (MOS6502::not_implemented, AddressingMode::Implied), // D2
                (MOS6502::not_implemented, AddressingMode::Implied), // D3
                (MOS6502::not_implemented, AddressingMode::Implied), // D4
                (MOS6502::not_implemented, AddressingMode::Implied), // D5
                (MOS6502::not_implemented, AddressingMode::Implied), // D6
                (MOS6502::not_implemented, AddressingMode::Implied), // D7
                (MOS6502::not_implemented, AddressingMode::Implied), // D8
                (MOS6502::not_implemented, AddressingMode::Implied), // D9
                (MOS6502::not_implemented, AddressingMode::Implied), // DA
                (MOS6502::not_implemented, AddressingMode::Implied), // DB
                (MOS6502::not_implemented, AddressingMode::Implied), // DC
                (MOS6502::not_implemented, AddressingMode::Implied), // DD
                (MOS6502::not_implemented, AddressingMode::Implied), // DE
                (MOS6502::not_implemented, AddressingMode::Implied), // DF
                (MOS6502::not_implemented, AddressingMode::Implied), // E0
                (MOS6502::not_implemented, AddressingMode::Implied), // E1
                (MOS6502::not_implemented, AddressingMode::Implied), // E2
                (MOS6502::not_implemented, AddressingMode::Implied), // E3
                (MOS6502::not_implemented, AddressingMode::Implied), // E4
                (MOS6502::not_implemented, AddressingMode::Implied), // E5
                (MOS6502::not_implemented, AddressingMode::Implied), // E6
                (MOS6502::not_implemented, AddressingMode::Implied), // E7
                (MOS6502::not_implemented, AddressingMode::Implied), // E8
                (MOS6502::not_implemented, AddressingMode::Implied), // E9
                (MOS6502::not_implemented, AddressingMode::Implied), // EA
                (MOS6502::not_implemented, AddressingMode::Implied), // EB
                (MOS6502::not_implemented, AddressingMode::Implied), // EC
                (MOS6502::not_implemented, AddressingMode::Implied), // ED
                (MOS6502::not_implemented, AddressingMode::Implied), // EE
                (MOS6502::not_implemented, AddressingMode::Implied), // EF
                (MOS6502::not_implemented, AddressingMode::Implied), // F0
                (MOS6502::not_implemented, AddressingMode::Implied), // F1
                (MOS6502::not_implemented, AddressingMode::Implied), // F2
                (MOS6502::not_implemented, AddressingMode::Implied), // F3
                (MOS6502::not_implemented, AddressingMode::Implied), // F4
                (MOS6502::not_implemented, AddressingMode::Implied), // F5
                (MOS6502::not_implemented, AddressingMode::Implied), // F6
                (MOS6502::not_implemented, AddressingMode::Implied), // F7
                (MOS6502::not_implemented, AddressingMode::Implied), // F8
                (MOS6502::not_implemented, AddressingMode::Implied), // F9
                (MOS6502::not_implemented, AddressingMode::Implied), // FA
                (MOS6502::not_implemented, AddressingMode::Implied), // FB
                (MOS6502::not_implemented, AddressingMode::Implied), // FC
                (MOS6502::not_implemented, AddressingMode::Implied), // FD
                (MOS6502::not_implemented, AddressingMode::Implied), // FE
                (MOS6502::not_implemented, AddressingMode::Implied), // FF
            ],
        }
    }

    /// Change value of program counter
    pub fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }

    /// Return number of elapsed CPU cycles
    pub fn get_cycles(&self) -> u128 {
        self.cycles
    }

    /// Start CPU
    pub fn run(&mut self) {
        let mut opc: u8;
        loop {
            opc = self.bus.read(self.program_counter);
            let (opcode_func, address_mode) = self.opcode_vec[opc as usize];
            opcode_func(self, address_mode);
        }
    }

    /// Run CPU for a specific number of cycles
    pub fn run_for_cycles(&mut self, cycles: u128) {
        let mut opc: u8;
        while self.cycles < cycles {
            opc = self.bus.read(self.program_counter);
            let (opcode_func, address_mode) = self.opcode_vec[opc as usize];
            opcode_func(self, address_mode);
        }
    }

    pub fn get_bus(&mut self) -> &mut impl Bus {
        &mut self.bus
    }

    /// Check if specified flag is set
    pub fn flag_check(&self, f: u8) -> bool {
        self.status_register & f != 0
    }

    pub fn get_accumulator(&self) -> u8 {
        self.accumulator
    }

    pub fn get_x_register(&self) -> u8 {
        self.x_register
    }

    pub fn get_y_register(&self) -> u8 {
        self.y_register
    }

    /// Turn specified flag on/off
    fn flag_toggle(&mut self, f: u8, value: bool) {
        if value {
            self.status_register |= f; // set flag
        } else {
            self.status_register &= !f; // clear flag
        }
    }

    // load value into accumulator
    fn lda(&mut self, address_mode: AddressingMode) {
        self.cycles = self.cycles.wrapping_add(1);
        let operand = self.resolve_operand(address_mode);
        self.accumulator = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDA");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.accumulator == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.accumulator & 0b10000000 != 0);

        self.program_counter = self.program_counter.wrapping_add(1);
    }

    // load value into X register
    fn ldx(&mut self, address_mode: AddressingMode) {
        self.cycles = self.cycles.wrapping_add(1);
        let operand = self.resolve_operand(address_mode);
        self.x_register = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDX");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.x_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.x_register & 0b10000000 != 0);

        self.program_counter = self.program_counter.wrapping_add(1);
    }

    // load value into Y register
    fn ldy(&mut self, address_mode: AddressingMode) {
        self.cycles = self.cycles.wrapping_add(1);
        let operand = self.resolve_operand(address_mode);
        self.y_register = match operand {
            OpcodeOperand::Byte(b) => b,
            OpcodeOperand::Address(addr) => self.bus.read(addr),
            _ => {
                panic!("Invalid addressing mode for LDY");
            }
        };

        self.flag_toggle(FLAG_ZERO, self.y_register == 0);
        self.flag_toggle(FLAG_NEGATIVE, self.y_register & 0b10000000 != 0);

        self.program_counter = self.program_counter.wrapping_add(1);
    }

    // add to accumulator with carry
    fn adc(&mut self, address_mode: AddressingMode) {
        let a_oldvalue = self.accumulator;
        let operand = self.resolve_operand(address_mode);
        let value = match operand {
            OpcodeOperand::Byte(b) => self.bus.read(b as u16),
            _ => {
                panic!("Invalid addressing mode for ADC");
            }
        };
        self.accumulator = self.accumulator.wrapping_add(value);
        self.accumulator = self
            .accumulator
            .wrapping_add(self.flag_check(FLAG_CARRY) as u8);
        self.flag_toggle(FLAG_OVERFLOW, self.accumulator < a_oldvalue);
    }

    fn not_implemented(&mut self, _: AddressingMode) {
        panic!("Opcode not implemented.")
    }

    /// Given some addressing mode, returns operand and increases CPU cycles as appropriate
    fn resolve_operand(&mut self, address_mode: AddressingMode) -> OpcodeOperand {
        match address_mode {
            AddressingMode::Accumulator => {
                self.cycles = self.cycles.wrapping_add(1);
                OpcodeOperand::Byte(self.accumulator)
            }
            AddressingMode::Absolute => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let low_byte: u8 = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let high_byte: u8 = self.bus.read(self.program_counter);

                let addr = u16::from_le_bytes([low_byte, high_byte]);

                self.cycles = self.cycles.wrapping_add(2);
                OpcodeOperand::Address(addr)
            }
            AddressingMode::AbsoluteXIndex => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let low_byte: u8 = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let high_byte: u8 = self.bus.read(self.program_counter);

                let mut addr =
                    u16::from_le_bytes([low_byte, high_byte]).wrapping_add(self.x_register as u16);

                self.cycles = self.cycles.wrapping_add(2);
                if self.flag_check(FLAG_CARRY) {
                    let old_addr = addr;
                    addr = addr.wrapping_add(1);
                    // add one more cycle if page boundaries were crossed
                    if old_addr & 0xFF00 != addr & 0xFF00 {
                        self.cycles = self.cycles.wrapping_add(1);
                    }
                }

                OpcodeOperand::Byte(self.bus.read(addr))
            }
            AddressingMode::AbsoluteYIndex => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let low_byte: u8 = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let high_byte: u8 = self.bus.read(self.program_counter);

                let mut addr =
                    u16::from_le_bytes([low_byte, high_byte]).wrapping_add(self.y_register as u16);

                self.cycles = self.cycles.wrapping_add(2);
                if self.flag_check(FLAG_CARRY) {
                    let old_addr = addr;
                    addr = addr.wrapping_add(1);
                    // add one more cycle if page boundaries were crossed
                    if old_addr & 0xFF00 != addr & 0xFF00 {
                        self.cycles = self.cycles.wrapping_add(1);
                    }
                }

                OpcodeOperand::Byte(self.bus.read(addr))
            }
            AddressingMode::Immediate => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let byte: u8 = self.bus.read(self.program_counter);

                self.cycles = self.cycles.wrapping_add(1);
                OpcodeOperand::Byte(byte)
            }
            AddressingMode::Implied => OpcodeOperand::None,
            AddressingMode::Indirect => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let mut low_byte: u8 = self.bus.read(self.program_counter);
                self.program_counter = self.program_counter.wrapping_add(1);
                let mut high_byte: u8 = self.bus.read(self.program_counter);

                let addr = u16::from_le_bytes([low_byte, high_byte]);

                low_byte = self.bus.read(addr);
                high_byte = self.bus.read(addr.wrapping_add(1));

                self.cycles = self.cycles.wrapping_add(2);
                OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]))
            }
            AddressingMode::XIndexIndirect => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let mut zp_addr: u16 = self.bus.read(self.program_counter) as u16;

                zp_addr = zp_addr.wrapping_add(self.x_register as u16);

                let low_byte = self.bus.read(zp_addr);
                let high_byte = self.bus.read(zp_addr.wrapping_add(1));

                self.cycles = self.cycles.wrapping_add(6);
                OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]))
            }
            AddressingMode::IndirectYIndex => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let zp_addr: u16 = self.bus.read(self.program_counter) as u16;

                let low_byte = self.bus.read(zp_addr);
                let high_byte = self.bus.read(zp_addr.wrapping_add(1));

                self.cycles = self.cycles.wrapping_add(6);
                OpcodeOperand::Address(
                    u16::from_le_bytes([low_byte, high_byte]).wrapping_add(self.y_register as u16),
                )
            }
            AddressingMode::Relative => {
                self.program_counter = self.program_counter.wrapping_add(1);
                let offset = self.bus.read(self.program_counter) as i16;

                let addr: u16 = self.program_counter.wrapping_add_signed(offset);

                OpcodeOperand::Address(addr)
            }
            AddressingMode::Zeropage => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.cycles = self.cycles.wrapping_add(1);

                let zp_addr = self.bus.read(self.program_counter) as u16;
                OpcodeOperand::Address(zp_addr)
            }
            AddressingMode::ZeropageXIndex => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.cycles = self.cycles.wrapping_add(1);

                let offset = self.x_register as u16;
                let zp_addr = self.bus.read(self.program_counter) as u16;
                let addr: u16 = zp_addr.wrapping_add(offset);

                OpcodeOperand::Address(addr)
            }
            AddressingMode::ZeropageYIndex => {
                self.program_counter = self.program_counter.wrapping_add(1);
                self.cycles = self.cycles.wrapping_add(1);

                let offset = self.y_register as u16;
                let zp_addr = self.bus.read(self.program_counter) as u16;
                let addr: u16 = zp_addr.wrapping_add(offset);

                OpcodeOperand::Address(addr)
            }
        }
    }
}
