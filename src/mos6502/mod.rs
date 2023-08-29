mod arithmetic_ops;
mod branch_ops;
mod comparison_ops;
mod dec_and_inc_ops;
mod flag_ops;
mod interrupt_ops;
mod jump_ops;
mod logical_ops;
mod other_ops;
mod shift_and_rotate_ops;
mod stack_ops;
mod transfer_ops;

use serde::{Deserialize, Serialize};

use crate::error::*;

pub trait Bus {
    fn read(&self, address: u16) -> Result<u8, BusError>;
    fn write(&mut self, address: u16, value: u8) -> Result<(), BusError>;
}

pub const FLAG_NEGATIVE: u8 = 1 << 7;
pub const FLAG_OVERFLOW: u8 = 1 << 6;
pub const FLAG_BREAK: u8 = 1 << 4;
pub const FLAG_DECIMAL: u8 = 1 << 3;
pub const FLAG_NO_INTERRUPTS: u8 = 1 << 2;
pub const FLAG_ZERO: u8 = 1 << 1;
pub const FLAG_CARRY: u8 = 1 << 0;

const STACK_BASE: u16 = 0x0100;

const NEGATIVE_BIT_MASK: u8 = 0b10000000;

enum InterruptKind {
    Nmi,
    Irq,
}

type OpcodeFunction<T> = fn(&mut MOS6502<T>, &mut T, AddressingMode) -> Result<(), EmulationError>;
struct OpcodeFunctionArray<T: Bus>([(OpcodeFunction<T>, AddressingMode); 256]);

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

#[derive(Serialize, Deserialize)]
pub struct MOS6502<T: Bus> {
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    status_register: u8,
    program_counter: u16,
    cycles: u64,
    #[serde(skip_serializing, skip_deserializing)]
    opcode_array: OpcodeFunctionArray<T>,
    irq: bool,
    nmi: bool,
}

impl<T: Bus> Default for OpcodeFunctionArray<T> {
    fn default() -> Self {
        OpcodeFunctionArray([
            (MOS6502::brk, AddressingMode::Implied),             // 00
            (MOS6502::ora, AddressingMode::XIndexIndirect),      // 01
            (MOS6502::not_implemented, AddressingMode::Implied), // 02
            (MOS6502::not_implemented, AddressingMode::Implied), // 03
            (MOS6502::not_implemented, AddressingMode::Implied), // 04
            (MOS6502::ora, AddressingMode::Zeropage),            // 05
            (MOS6502::asl, AddressingMode::Zeropage),            // 06
            (MOS6502::not_implemented, AddressingMode::Implied), // 07
            (MOS6502::php, AddressingMode::Implied),             // 08
            (MOS6502::ora, AddressingMode::Immediate),           // 09
            (MOS6502::asl, AddressingMode::Accumulator),         // 0A
            (MOS6502::not_implemented, AddressingMode::Implied), // 0B
            (MOS6502::not_implemented, AddressingMode::Implied), // 0C
            (MOS6502::ora, AddressingMode::Absolute),            // 0D
            (MOS6502::asl, AddressingMode::Absolute),            // 0E
            (MOS6502::not_implemented, AddressingMode::Implied), // 0F
            (MOS6502::bpl, AddressingMode::Relative),            // 10
            (MOS6502::ora, AddressingMode::IndirectYIndex),      // 11
            (MOS6502::not_implemented, AddressingMode::Implied), // 12
            (MOS6502::not_implemented, AddressingMode::Implied), // 13
            (MOS6502::not_implemented, AddressingMode::Implied), // 14
            (MOS6502::ora, AddressingMode::ZeropageXIndex),      // 15
            (MOS6502::asl, AddressingMode::ZeropageXIndex),      // 16
            (MOS6502::not_implemented, AddressingMode::Implied), // 17
            (MOS6502::clc, AddressingMode::Implied),             // 18
            (MOS6502::ora, AddressingMode::AbsoluteYIndex),      // 19
            (MOS6502::not_implemented, AddressingMode::Implied), // 1A
            (MOS6502::not_implemented, AddressingMode::Implied), // 1B
            (MOS6502::not_implemented, AddressingMode::Implied), // 1C
            (MOS6502::ora, AddressingMode::AbsoluteXIndex),      // 1D
            (MOS6502::asl, AddressingMode::AbsoluteXIndex),      // 1E
            (MOS6502::not_implemented, AddressingMode::Implied), // 1F
            (MOS6502::jsr, AddressingMode::Absolute),            // 20
            (MOS6502::and, AddressingMode::XIndexIndirect),      // 21
            (MOS6502::not_implemented, AddressingMode::Implied), // 22
            (MOS6502::not_implemented, AddressingMode::Implied), // 23
            (MOS6502::bit, AddressingMode::Zeropage),            // 24
            (MOS6502::and, AddressingMode::Zeropage),            // 25
            (MOS6502::rol, AddressingMode::Zeropage),            // 26
            (MOS6502::not_implemented, AddressingMode::Implied), // 27
            (MOS6502::plp, AddressingMode::Implied),             // 28
            (MOS6502::and, AddressingMode::Immediate),           // 29
            (MOS6502::rol, AddressingMode::Accumulator),         // 2A
            (MOS6502::not_implemented, AddressingMode::Implied), // 2B
            (MOS6502::bit, AddressingMode::Absolute),            // 2C
            (MOS6502::and, AddressingMode::Absolute),            // 2D
            (MOS6502::rol, AddressingMode::Absolute),            // 2E
            (MOS6502::not_implemented, AddressingMode::Implied), // 2F
            (MOS6502::bmi, AddressingMode::Relative),            // 30
            (MOS6502::and, AddressingMode::IndirectYIndex),      // 31
            (MOS6502::not_implemented, AddressingMode::Implied), // 32
            (MOS6502::not_implemented, AddressingMode::Implied), // 33
            (MOS6502::not_implemented, AddressingMode::Implied), // 34
            (MOS6502::and, AddressingMode::ZeropageXIndex),      // 35
            (MOS6502::rol, AddressingMode::ZeropageXIndex),      // 36
            (MOS6502::not_implemented, AddressingMode::Implied), // 37
            (MOS6502::sec, AddressingMode::Implied),             // 38
            (MOS6502::and, AddressingMode::AbsoluteYIndex),      // 39
            (MOS6502::not_implemented, AddressingMode::Implied), // 3A
            (MOS6502::not_implemented, AddressingMode::Implied), // 3B
            (MOS6502::not_implemented, AddressingMode::Implied), // 3C
            (MOS6502::and, AddressingMode::AbsoluteXIndex),      // 3D
            (MOS6502::rol, AddressingMode::AbsoluteXIndex),      // 3E
            (MOS6502::not_implemented, AddressingMode::Implied), // 3F
            (MOS6502::rti, AddressingMode::Implied),             // 40
            (MOS6502::eor, AddressingMode::XIndexIndirect),      // 41
            (MOS6502::not_implemented, AddressingMode::Implied), // 42
            (MOS6502::not_implemented, AddressingMode::Implied), // 43
            (MOS6502::not_implemented, AddressingMode::Implied), // 44
            (MOS6502::eor, AddressingMode::Zeropage),            // 45
            (MOS6502::lsr, AddressingMode::Zeropage),            // 46
            (MOS6502::not_implemented, AddressingMode::Implied), // 47
            (MOS6502::pha, AddressingMode::Implied),             // 48
            (MOS6502::eor, AddressingMode::Immediate),           // 49
            (MOS6502::lsr, AddressingMode::Accumulator),         // 4A
            (MOS6502::not_implemented, AddressingMode::Implied), // 4B
            (MOS6502::jmp, AddressingMode::Absolute),            // 4C
            (MOS6502::eor, AddressingMode::Absolute),            // 4D
            (MOS6502::lsr, AddressingMode::Absolute),            // 4E
            (MOS6502::not_implemented, AddressingMode::Implied), // 4F
            (MOS6502::bvc, AddressingMode::Relative),            // 50
            (MOS6502::eor, AddressingMode::IndirectYIndex),      // 51
            (MOS6502::not_implemented, AddressingMode::Implied), // 52
            (MOS6502::not_implemented, AddressingMode::Implied), // 53
            (MOS6502::not_implemented, AddressingMode::Implied), // 54
            (MOS6502::eor, AddressingMode::ZeropageXIndex),      // 55
            (MOS6502::lsr, AddressingMode::ZeropageXIndex),      // 56
            (MOS6502::not_implemented, AddressingMode::Implied), // 57
            (MOS6502::cli, AddressingMode::Implied),             // 58
            (MOS6502::eor, AddressingMode::AbsoluteYIndex),      // 59
            (MOS6502::not_implemented, AddressingMode::Implied), // 5A
            (MOS6502::not_implemented, AddressingMode::Implied), // 5B
            (MOS6502::not_implemented, AddressingMode::Implied), // 5C
            (MOS6502::eor, AddressingMode::AbsoluteXIndex),      // 5D
            (MOS6502::lsr, AddressingMode::AbsoluteXIndex),      // 5E
            (MOS6502::not_implemented, AddressingMode::Implied), // 5F
            (MOS6502::rts, AddressingMode::Implied),             // 60
            (MOS6502::adc, AddressingMode::XIndexIndirect),      // 61
            (MOS6502::not_implemented, AddressingMode::Implied), // 62
            (MOS6502::not_implemented, AddressingMode::Implied), // 63
            (MOS6502::not_implemented, AddressingMode::Implied), // 64
            (MOS6502::adc, AddressingMode::Zeropage),            // 65
            (MOS6502::ror, AddressingMode::Zeropage),            // 66
            (MOS6502::not_implemented, AddressingMode::Implied), // 67
            (MOS6502::pla, AddressingMode::Implied),             // 68
            (MOS6502::adc, AddressingMode::Immediate),           // 69
            (MOS6502::ror, AddressingMode::Accumulator),         // 6A
            (MOS6502::not_implemented, AddressingMode::Implied), // 6B
            (MOS6502::jmp, AddressingMode::Indirect),            // 6C
            (MOS6502::adc, AddressingMode::Absolute),            // 6D
            (MOS6502::ror, AddressingMode::Absolute),            // 6E
            (MOS6502::not_implemented, AddressingMode::Implied), // 6F
            (MOS6502::bvs, AddressingMode::Relative),            // 70
            (MOS6502::adc, AddressingMode::IndirectYIndex),      // 71
            (MOS6502::not_implemented, AddressingMode::Implied), // 72
            (MOS6502::not_implemented, AddressingMode::Implied), // 73
            (MOS6502::not_implemented, AddressingMode::Implied), // 74
            (MOS6502::adc, AddressingMode::ZeropageXIndex),      // 75
            (MOS6502::ror, AddressingMode::ZeropageXIndex),      // 76
            (MOS6502::not_implemented, AddressingMode::Implied), // 77
            (MOS6502::sei, AddressingMode::Implied),             // 78
            (MOS6502::adc, AddressingMode::AbsoluteYIndex),      // 79
            (MOS6502::not_implemented, AddressingMode::Implied), // 7A
            (MOS6502::not_implemented, AddressingMode::Implied), // 7B
            (MOS6502::not_implemented, AddressingMode::Implied), // 7C
            (MOS6502::adc, AddressingMode::AbsoluteXIndex),      // 7D
            (MOS6502::ror, AddressingMode::AbsoluteXIndex),      // 7E
            (MOS6502::not_implemented, AddressingMode::Implied), // 7F
            (MOS6502::not_implemented, AddressingMode::Implied), // 80
            (MOS6502::sta, AddressingMode::XIndexIndirect),      // 81
            (MOS6502::not_implemented, AddressingMode::Implied), // 82
            (MOS6502::not_implemented, AddressingMode::Implied), // 83
            (MOS6502::sty, AddressingMode::Zeropage),            // 84
            (MOS6502::sta, AddressingMode::Zeropage),            // 85
            (MOS6502::stx, AddressingMode::Zeropage),            // 86
            (MOS6502::not_implemented, AddressingMode::Implied), // 87
            (MOS6502::dey, AddressingMode::Implied),             // 88
            (MOS6502::not_implemented, AddressingMode::Implied), // 89
            (MOS6502::txa, AddressingMode::Implied),             // 8A
            (MOS6502::not_implemented, AddressingMode::Implied), // 8B
            (MOS6502::sty, AddressingMode::Absolute),            // 8C
            (MOS6502::sta, AddressingMode::Absolute),            // 8D
            (MOS6502::stx, AddressingMode::Absolute),            // 8E
            (MOS6502::not_implemented, AddressingMode::Implied), // 8F
            (MOS6502::bcc, AddressingMode::Relative),            // 90
            (MOS6502::sta, AddressingMode::IndirectYIndex),      // 91
            (MOS6502::not_implemented, AddressingMode::Implied), // 92
            (MOS6502::not_implemented, AddressingMode::Implied), // 93
            (MOS6502::sty, AddressingMode::ZeropageXIndex),      // 94
            (MOS6502::sta, AddressingMode::ZeropageXIndex),      // 95
            (MOS6502::stx, AddressingMode::ZeropageYIndex),      // 96
            (MOS6502::not_implemented, AddressingMode::Implied), // 97
            (MOS6502::tya, AddressingMode::Implied),             // 98
            (MOS6502::sta, AddressingMode::AbsoluteYIndex),      // 99
            (MOS6502::txs, AddressingMode::Implied),             // 9A
            (MOS6502::not_implemented, AddressingMode::Implied), // 9B
            (MOS6502::not_implemented, AddressingMode::Implied), // 9C
            (MOS6502::sta, AddressingMode::AbsoluteXIndex),      // 9D
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
            (MOS6502::tay, AddressingMode::Implied),             // A8
            (MOS6502::lda, AddressingMode::Immediate),           // A9
            (MOS6502::tax, AddressingMode::Implied),             // AA
            (MOS6502::not_implemented, AddressingMode::Implied), // AB
            (MOS6502::ldy, AddressingMode::Absolute),            // AC
            (MOS6502::lda, AddressingMode::Absolute),            // AD
            (MOS6502::ldx, AddressingMode::Absolute),            // AE
            (MOS6502::not_implemented, AddressingMode::Implied), // AF
            (MOS6502::bcs, AddressingMode::Relative),            // B0
            (MOS6502::lda, AddressingMode::IndirectYIndex),      // B1
            (MOS6502::not_implemented, AddressingMode::Implied), // B2
            (MOS6502::not_implemented, AddressingMode::Implied), // B3
            (MOS6502::ldy, AddressingMode::ZeropageXIndex),      // B4
            (MOS6502::lda, AddressingMode::ZeropageXIndex),      // B5
            (MOS6502::ldx, AddressingMode::ZeropageYIndex),      // B6
            (MOS6502::not_implemented, AddressingMode::Implied), // B7
            (MOS6502::clv, AddressingMode::Implied),             // B8
            (MOS6502::lda, AddressingMode::AbsoluteYIndex),      // B9
            (MOS6502::tsx, AddressingMode::Implied),             // BA
            (MOS6502::not_implemented, AddressingMode::Implied), // BB
            (MOS6502::ldy, AddressingMode::AbsoluteXIndex),      // BC
            (MOS6502::lda, AddressingMode::AbsoluteXIndex),      // BD
            (MOS6502::ldx, AddressingMode::AbsoluteYIndex),      // BE
            (MOS6502::not_implemented, AddressingMode::Implied), // BF
            (MOS6502::cpy, AddressingMode::Immediate),           // C0
            (MOS6502::cmp, AddressingMode::XIndexIndirect),      // C1
            (MOS6502::not_implemented, AddressingMode::Implied), // C2
            (MOS6502::not_implemented, AddressingMode::Implied), // C3
            (MOS6502::cpy, AddressingMode::Zeropage),            // C4
            (MOS6502::cmp, AddressingMode::Zeropage),            // C5
            (MOS6502::dec, AddressingMode::Zeropage),            // C6
            (MOS6502::not_implemented, AddressingMode::Implied), // C7
            (MOS6502::iny, AddressingMode::Implied),             // C8
            (MOS6502::cmp, AddressingMode::Immediate),           // C9
            (MOS6502::dex, AddressingMode::Implied),             // CA
            (MOS6502::not_implemented, AddressingMode::Implied), // CB
            (MOS6502::cpy, AddressingMode::Absolute),            // CC
            (MOS6502::cmp, AddressingMode::Absolute),            // CD
            (MOS6502::dec, AddressingMode::Absolute),            // CE
            (MOS6502::not_implemented, AddressingMode::Implied), // CF
            (MOS6502::bne, AddressingMode::Relative),            // D0
            (MOS6502::cmp, AddressingMode::IndirectYIndex),      // D1
            (MOS6502::not_implemented, AddressingMode::Implied), // D2
            (MOS6502::not_implemented, AddressingMode::Implied), // D3
            (MOS6502::not_implemented, AddressingMode::Implied), // D4
            (MOS6502::cmp, AddressingMode::ZeropageXIndex),      // D5
            (MOS6502::dec, AddressingMode::ZeropageXIndex),      // D6
            (MOS6502::not_implemented, AddressingMode::Implied), // D7
            (MOS6502::cld, AddressingMode::Implied),             // D8
            (MOS6502::cmp, AddressingMode::AbsoluteYIndex),      // D9
            (MOS6502::not_implemented, AddressingMode::Implied), // DA
            (MOS6502::not_implemented, AddressingMode::Implied), // DB
            (MOS6502::not_implemented, AddressingMode::Implied), // DC
            (MOS6502::cmp, AddressingMode::AbsoluteXIndex),      // DD
            (MOS6502::dec, AddressingMode::AbsoluteXIndex),      // DE
            (MOS6502::not_implemented, AddressingMode::Implied), // DF
            (MOS6502::cpx, AddressingMode::Immediate),           // E0
            (MOS6502::sbc, AddressingMode::XIndexIndirect),      // E1
            (MOS6502::not_implemented, AddressingMode::Implied), // E2
            (MOS6502::not_implemented, AddressingMode::Implied), // E3
            (MOS6502::cpx, AddressingMode::Zeropage),            // E4
            (MOS6502::sbc, AddressingMode::Zeropage),            // E5
            (MOS6502::inc, AddressingMode::Zeropage),            // E6
            (MOS6502::not_implemented, AddressingMode::Implied), // E7
            (MOS6502::inx, AddressingMode::Implied),             // E8
            (MOS6502::sbc, AddressingMode::Immediate),           // E9
            (MOS6502::nop, AddressingMode::Implied),             // EA
            (MOS6502::not_implemented, AddressingMode::Implied), // EB
            (MOS6502::cpx, AddressingMode::Absolute),            // EC
            (MOS6502::sbc, AddressingMode::Absolute),            // ED
            (MOS6502::inc, AddressingMode::Absolute),            // EE
            (MOS6502::not_implemented, AddressingMode::Implied), // EF
            (MOS6502::beq, AddressingMode::Relative),            // F0
            (MOS6502::sbc, AddressingMode::IndirectYIndex),      // F1
            (MOS6502::not_implemented, AddressingMode::Implied), // F2
            (MOS6502::not_implemented, AddressingMode::Implied), // F3
            (MOS6502::not_implemented, AddressingMode::Implied), // F4
            (MOS6502::sbc, AddressingMode::ZeropageXIndex),      // F5
            (MOS6502::inc, AddressingMode::ZeropageXIndex),      // F6
            (MOS6502::not_implemented, AddressingMode::Implied), // F7
            (MOS6502::sed, AddressingMode::Implied),             // F8
            (MOS6502::sbc, AddressingMode::AbsoluteYIndex),      // F9
            (MOS6502::not_implemented, AddressingMode::Implied), // FA
            (MOS6502::not_implemented, AddressingMode::Implied), // FB
            (MOS6502::not_implemented, AddressingMode::Implied), // FC
            (MOS6502::sbc, AddressingMode::AbsoluteXIndex),      // FD
            (MOS6502::inc, AddressingMode::AbsoluteXIndex),      // FE
            (MOS6502::not_implemented, AddressingMode::Implied), // FF
        ])
    }
}

impl<T: Bus> MOS6502<T> {
    /// Create new instance of MOS6502
    pub fn new() -> Result<Self, EmulationError> {
        Ok(Self {
            accumulator: u8::MIN,
            x_register: u8::MIN,
            y_register: u8::MIN,
            program_counter: u16::MIN,
            stack_pointer: u8::MAX,
            status_register: (1 << 5) | FLAG_BREAK,
            cycles: u64::MIN,
            opcode_array: OpcodeFunctionArray::default(),
            nmi: false,
            irq: false,
        })
    }

    #[inline]
    pub fn program_counter(&self) -> u16 {
        self.program_counter
    }

    #[inline]
    pub fn accumulator(&self) -> u8 {
        self.accumulator
    }

    #[inline]
    pub fn x_register(&self) -> u8 {
        self.x_register
    }

    #[inline]
    pub fn y_register(&self) -> u8 {
        self.y_register
    }

    #[inline]
    pub fn stack_pointer(&self) -> u8 {
        self.stack_pointer
    }

    /// Change value of program counter
    #[inline]
    pub fn set_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }

    /// Return number of elapsed CPU cycles
    #[inline]
    pub fn cycles(&self) -> u64 {
        self.cycles
    }

    #[inline]
    fn pop_from_stack(&mut self, bus: &T) -> Result<u8, BusError> {
        self.stack_pointer = self.stack_pointer.wrapping_add(1);
        bus.read(STACK_BASE + self.stack_pointer as u16)
    }

    #[inline]
    fn push_to_stack(&mut self, bus: &mut T, value: u8) -> Result<(), BusError> {
        let result = bus.write(STACK_BASE + self.stack_pointer as u16, value);
        self.stack_pointer = self.stack_pointer.wrapping_sub(1);
        result
    }

    fn perform_interrupt(
        &mut self,
        return_address: u16,
        kind: InterruptKind,
        bus: &mut T,
    ) -> Result<(), EmulationError> {
        let (return_address_lo, return_address_hi): (u8, u8) = return_address.to_le_bytes().into();

        self.push_to_stack(bus, return_address_hi)?;
        self.push_to_stack(bus, return_address_lo)?;

        let (vector_address, status_register_value): (u16, u8) = match kind {
            InterruptKind::Irq => (0xFFFE, self.status_register | FLAG_BREAK),
            InterruptKind::Nmi => (0xFFFA, self.status_register & !FLAG_BREAK),
        };

        self.push_to_stack(bus, status_register_value)?;

        let divert_address_lo = bus.read(vector_address)?;
        let divert_address_hi = bus.read(vector_address + 1)?;

        self.set_program_counter(u16::from_le_bytes([divert_address_lo, divert_address_hi]));

        self.increment_cycles(7);
        Ok(())
    }

    pub fn handle_interrupts(&mut self, bus: &mut T) -> Result<(), EmulationError> {
        if self.nmi {
            self.nmi = false;
            self.perform_interrupt(self.program_counter, InterruptKind::Nmi, bus)?;
        }
        if self.irq {
            self.irq = false;
            if !self.flag_check(FLAG_NO_INTERRUPTS) {
                self.perform_interrupt(self.program_counter, InterruptKind::Irq, bus)?;
            }
        }
        Ok(())
    }

    fn not_implemented(
        &mut self,
        _: &mut impl Bus,
        _: AddressingMode,
    ) -> Result<(), EmulationError> {
        Err(EmulationError::OpcodeNotImplemented)
    }

    /// Step over one CPU instruction
    pub fn step(&mut self, bus: &mut T) -> Result<(), EmulationError> {
        let opc = bus.read(self.program_counter)?;
        self.increment_program_counter(1);
        let (ref opcode_func, address_mode) = self.opcode_array.0[opc as usize];
        let result = opcode_func(self, bus, address_mode);
        self.handle_interrupts(bus)?;
        result
    }

    /// Check if specified flag is set
    #[inline]
    pub fn flag_check(&self, flag: u8) -> bool {
        self.status_register & flag != 0
    }

    /// Turn specified flag on/off
    #[inline]
    fn flag_toggle(&mut self, f: u8, value: bool) {
        if value {
            self.status_register |= f; // set flag
        } else {
            self.status_register &= !f; // clear flag
        }
    }

    #[inline]
    fn increment_program_counter(&mut self, n: u16) {
        self.set_program_counter(self.program_counter.wrapping_add(n));
    }

    #[inline]
    fn increment_cycles(&mut self, n: u64) {
        self.cycles = self.cycles.wrapping_add(n);
    }

    /// Given some addressing mode, returns operand and increases CPU cycles as appropriate
    #[inline]
    fn resolve_operand(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<OpcodeOperand, EmulationError> {
        match address_mode {
            AddressingMode::Accumulator => {
                self.increment_cycles(1);
                Ok(OpcodeOperand::Byte(self.accumulator))
            }
            AddressingMode::Absolute => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let addr = u16::from_le_bytes([low_byte, high_byte]);

                self.increment_cycles(3);
                Ok(OpcodeOperand::Address(addr))
            }
            AddressingMode::AbsoluteXIndex => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let (low_byte, overflow) = low_byte.overflowing_add(self.x_register);
                if overflow {
                    high_byte = high_byte.wrapping_add(1);
                    self.increment_cycles(1);
                }

                self.increment_cycles(3);
                let address = u16::from_le_bytes([low_byte, high_byte]);
                Ok(OpcodeOperand::Address(address))
            }
            AddressingMode::AbsoluteYIndex => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let (low_byte, overflow) = low_byte.overflowing_add(self.y_register);
                if overflow {
                    high_byte = high_byte.wrapping_add(1);
                    self.increment_cycles(1);
                }

                self.increment_cycles(3);
                let address = u16::from_le_bytes([low_byte, high_byte]);
                Ok(OpcodeOperand::Address(address))
            }
            AddressingMode::Immediate => {
                let byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                self.increment_cycles(1);
                Ok(OpcodeOperand::Byte(byte))
            }
            AddressingMode::Implied => Ok(OpcodeOperand::None),
            AddressingMode::Indirect => {
                let mut low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let addr = u16::from_le_bytes([low_byte, high_byte]);

                low_byte = bus.read(addr)?;
                high_byte = bus.read(addr.wrapping_add(1))?;

                self.increment_cycles(2);
                Ok(OpcodeOperand::Address(u16::from_le_bytes([
                    low_byte, high_byte,
                ])))
            }
            AddressingMode::XIndexIndirect => {
                let mut zp_addr: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                zp_addr = zp_addr.wrapping_add(self.x_register);

                let low_byte = bus.read(zp_addr as u16)?;
                let high_byte = bus.read(zp_addr.wrapping_add(1) as u16)?;

                self.increment_cycles(6);
                Ok(OpcodeOperand::Address(u16::from_le_bytes([
                    low_byte, high_byte,
                ])))
            }
            AddressingMode::IndirectYIndex => {
                let zp_addr = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let low_byte = bus.read(zp_addr as u16)?;
                let mut high_byte = bus.read(zp_addr.wrapping_add(1) as u16)?;

                let (low_byte, overflow) = low_byte.overflowing_add(self.y_register);
                if overflow {
                    high_byte = high_byte.wrapping_add(1);
                    self.increment_cycles(1);
                }

                self.increment_cycles(5);
                Ok(OpcodeOperand::Address(u16::from_le_bytes([
                    low_byte, high_byte,
                ])))
            }
            AddressingMode::Relative => {
                let offset = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let offset = (offset as i8) as i16;
                let new_pc = self.program_counter.wrapping_add_signed(offset);

                Ok(OpcodeOperand::Address(new_pc))
            }
            AddressingMode::Zeropage => {
                let zp_addr = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                self.increment_cycles(2);
                Ok(OpcodeOperand::Address(zp_addr as u16))
            }
            AddressingMode::ZeropageXIndex => {
                let offset = self.x_register;

                let zp_addr = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let addr = zp_addr.wrapping_add(offset);
                self.increment_cycles(3);

                Ok(OpcodeOperand::Address(addr as u16))
            }
            AddressingMode::ZeropageYIndex => {
                let offset = self.y_register;

                let zp_addr = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let addr = zp_addr.wrapping_add(offset);
                self.increment_cycles(3);

                Ok(OpcodeOperand::Address(addr as u16))
            }
        }
    }
}
