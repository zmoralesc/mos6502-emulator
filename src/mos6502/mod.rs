mod opcodes;

use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::error::*;

pub trait Bus {
    fn read(&mut self, address: u16) -> Result<u8, BusError>;
    fn write(&mut self, address: u16, value: u8) -> Result<(), BusError>;
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct CpuFlags(u8);

bitflags! {
    impl CpuFlags: u8 {
        const Negative = 1 << 7;
        const Overflow = 1 << 6;
        const Unused = 1 << 5;
        const Break = 1 << 4;
        const Decimal = 1 << 3;
        const NoInterrupts = 1 << 2;
        const Zero = 1 << 1;
        const Carry = 1 << 0;
    }
}

impl From<CpuFlags> for u8 {
    fn from(val: CpuFlags) -> Self {
        val.0
    }
}

impl From<u8> for CpuFlags {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

const STACK_BASE: u16 = 0x0100;

const NEGATIVE_BIT_MASK: u8 = 0b10000000;

enum InterruptKind {
    Nmi,
    Irq,
    Brk,
}

#[derive(Clone, Copy, Debug)]
enum Cycles {
    Fixed(u32),
    Variable(u32),
}

type OpcodeFunction<T> = fn(&mut MOS6502<T>, &mut T, AddressingMode) -> Result<u32, CpuError>;
struct OpcodeFunctionArray<T: Bus>([(OpcodeFunction<T>, AddressingMode, Cycles); 256]);

enum OpcodeOperand {
    Byte(u8),
    Address(u16),
    AddressWithOverflow(u16, bool),
    None,
}

#[derive(Clone, Copy, Debug)]
pub enum AddressingMode {
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
    status_register: CpuFlags,
    program_counter: u16,
    #[serde(skip_serializing, skip_deserializing)]
    opcode_array: OpcodeFunctionArray<T>,
}

impl<T: Bus> Default for OpcodeFunctionArray<T> {
    fn default() -> Self {
        OpcodeFunctionArray([
            (MOS6502::brk, AddressingMode::Implied, Cycles::Fixed(7)),              // 00
            (MOS6502::ora, AddressingMode::XIndexIndirect, Cycles::Fixed(6)),       // 01
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 02
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 03
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 04
            (MOS6502::ora, AddressingMode::Zeropage, Cycles::Fixed(3)),             // 05
            (MOS6502::asl, AddressingMode::Zeropage, Cycles::Fixed(5)),             // 06
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 07
            (MOS6502::php, AddressingMode::Implied, Cycles::Fixed(3)),              // 08
            (MOS6502::ora, AddressingMode::Immediate, Cycles::Fixed(2)),            // 09
            (MOS6502::asl, AddressingMode::Accumulator, Cycles::Fixed(2)),          // 0A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 0B
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 0C
            (MOS6502::ora, AddressingMode::Absolute, Cycles::Fixed(4)),             // 0D
            (MOS6502::asl, AddressingMode::Absolute, Cycles::Fixed(6)),             // 0E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 0F
            (MOS6502::bpl, AddressingMode::Relative, Cycles::Variable(2)),          // 10
            (MOS6502::ora, AddressingMode::IndirectYIndex, Cycles::Variable(5)),    // 11
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 12
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 13
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 14
            (MOS6502::ora, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // 15
            (MOS6502::asl, AddressingMode::ZeropageXIndex, Cycles::Fixed(6)),       // 16
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 17
            (MOS6502::clc, AddressingMode::Implied, Cycles::Fixed(2)),              // 18
            (MOS6502::ora, AddressingMode::AbsoluteYIndex, Cycles::Variable(4)),    // 19
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 1A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 1B
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 1C
            (MOS6502::ora, AddressingMode::AbsoluteXIndex, Cycles::Variable(4)),    // 1D
            (MOS6502::asl, AddressingMode::AbsoluteXIndex, Cycles::Fixed(7)),       // 1E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 1F
            (MOS6502::jsr, AddressingMode::Absolute, Cycles::Fixed(6)),             // 20
            (MOS6502::and, AddressingMode::XIndexIndirect, Cycles::Fixed(6)),       // 21
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 22
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 23
            (MOS6502::bit, AddressingMode::Zeropage, Cycles::Fixed(3)),             // 24
            (MOS6502::and, AddressingMode::Zeropage, Cycles::Fixed(3)),             // 25
            (MOS6502::rol, AddressingMode::Zeropage, Cycles::Fixed(5)),             // 26
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 27
            (MOS6502::plp, AddressingMode::Implied, Cycles::Fixed(4)),              // 28
            (MOS6502::and, AddressingMode::Immediate, Cycles::Fixed(2)),            // 29
            (MOS6502::rol, AddressingMode::Accumulator, Cycles::Fixed(2)),          // 2A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 2B
            (MOS6502::bit, AddressingMode::Absolute, Cycles::Fixed(4)),             // 2C
            (MOS6502::and, AddressingMode::Absolute, Cycles::Fixed(4)),             // 2D
            (MOS6502::rol, AddressingMode::Absolute, Cycles::Fixed(6)),             // 2E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 2F
            (MOS6502::bmi, AddressingMode::Relative, Cycles::Variable(2)),          // 30
            (MOS6502::and, AddressingMode::IndirectYIndex, Cycles::Variable(5)),    // 31
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 32
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 33
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 34
            (MOS6502::and, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // 35
            (MOS6502::rol, AddressingMode::ZeropageXIndex, Cycles::Fixed(6)),       // 36
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 37
            (MOS6502::sec, AddressingMode::Implied, Cycles::Fixed(2)),              // 38
            (MOS6502::and, AddressingMode::AbsoluteYIndex, Cycles::Variable(4)),    // 39
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 3A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 3B
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 3C
            (MOS6502::and, AddressingMode::AbsoluteXIndex, Cycles::Variable(4)),    // 3D
            (MOS6502::rol, AddressingMode::AbsoluteXIndex, Cycles::Fixed(7)),       // 3E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 3F
            (MOS6502::rti, AddressingMode::Implied, Cycles::Fixed(6)),              // 40
            (MOS6502::eor, AddressingMode::XIndexIndirect, Cycles::Fixed(6)),       // 41
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 42
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 43
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 44
            (MOS6502::eor, AddressingMode::Zeropage, Cycles::Fixed(3)),             // 45
            (MOS6502::lsr, AddressingMode::Zeropage, Cycles::Fixed(5)),             // 46
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 47
            (MOS6502::pha, AddressingMode::Implied, Cycles::Fixed(3)),              // 48
            (MOS6502::eor, AddressingMode::Immediate, Cycles::Fixed(2)),            // 49
            (MOS6502::lsr, AddressingMode::Accumulator, Cycles::Fixed(2)),          // 4A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 4B
            (MOS6502::jmp, AddressingMode::Absolute, Cycles::Fixed(3)),             // 4C
            (MOS6502::eor, AddressingMode::Absolute, Cycles::Fixed(4)),             // 4D
            (MOS6502::lsr, AddressingMode::Absolute, Cycles::Fixed(6)),             // 4E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 4F
            (MOS6502::bvc, AddressingMode::Relative, Cycles::Variable(2)),          // 50
            (MOS6502::eor, AddressingMode::IndirectYIndex, Cycles::Variable(5)),    // 51
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 52
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 53
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 54
            (MOS6502::eor, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // 55
            (MOS6502::lsr, AddressingMode::ZeropageXIndex, Cycles::Fixed(6)),       // 56
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 57
            (MOS6502::cli, AddressingMode::Implied, Cycles::Fixed(2)),              // 58
            (MOS6502::eor, AddressingMode::AbsoluteYIndex, Cycles::Variable(4)),    // 59
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 5A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 5B
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 5C
            (MOS6502::eor, AddressingMode::AbsoluteXIndex, Cycles::Variable(4)),    // 5D
            (MOS6502::lsr, AddressingMode::AbsoluteXIndex, Cycles::Fixed(7)),       // 5E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 5F
            (MOS6502::rts, AddressingMode::Implied, Cycles::Fixed(6)),              // 60
            (MOS6502::adc, AddressingMode::XIndexIndirect, Cycles::Fixed(6)),       // 61
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 62
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 63
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 64
            (MOS6502::adc, AddressingMode::Zeropage, Cycles::Fixed(3)),             // 65
            (MOS6502::ror, AddressingMode::Zeropage, Cycles::Fixed(5)),             // 66
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 67
            (MOS6502::pla, AddressingMode::Implied, Cycles::Fixed(4)),              // 68
            (MOS6502::adc, AddressingMode::Immediate, Cycles::Fixed(2)),            // 69
            (MOS6502::ror, AddressingMode::Accumulator, Cycles::Fixed(2)),          // 6A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 6B
            (MOS6502::jmp, AddressingMode::Indirect, Cycles::Fixed(5)),             // 6C
            (MOS6502::adc, AddressingMode::Absolute, Cycles::Fixed(4)),             // 6D
            (MOS6502::ror, AddressingMode::Absolute, Cycles::Fixed(6)),             // 6E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 6F
            (MOS6502::bvs, AddressingMode::Relative, Cycles::Variable(2)),          // 70
            (MOS6502::adc, AddressingMode::IndirectYIndex, Cycles::Variable(5)),    // 71
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 72
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 73
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 74
            (MOS6502::adc, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // 75
            (MOS6502::ror, AddressingMode::ZeropageXIndex, Cycles::Fixed(6)),       // 76
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 77
            (MOS6502::sei, AddressingMode::Implied, Cycles::Fixed(2)),              // 78
            (MOS6502::adc, AddressingMode::AbsoluteYIndex, Cycles::Variable(4)),    // 79
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 7A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 7B
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 7C
            (MOS6502::adc, AddressingMode::AbsoluteXIndex, Cycles::Variable(4)),    // 7D
            (MOS6502::ror, AddressingMode::AbsoluteXIndex, Cycles::Fixed(7)),       // 7E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 7F
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 80
            (MOS6502::sta, AddressingMode::XIndexIndirect, Cycles::Fixed(6)),       // 81
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 82
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 83
            (MOS6502::sty, AddressingMode::Zeropage, Cycles::Fixed(3)),             // 84
            (MOS6502::sta, AddressingMode::Zeropage, Cycles::Fixed(3)),             // 85
            (MOS6502::stx, AddressingMode::Zeropage, Cycles::Fixed(3)),             // 86
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 87
            (MOS6502::dey, AddressingMode::Implied, Cycles::Fixed(2)),              // 88
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 89
            (MOS6502::txa, AddressingMode::Implied, Cycles::Fixed(2)),              // 8A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 8B
            (MOS6502::sty, AddressingMode::Absolute, Cycles::Fixed(4)),             // 8C
            (MOS6502::sta, AddressingMode::Absolute, Cycles::Fixed(4)),             // 8D
            (MOS6502::stx, AddressingMode::Absolute, Cycles::Fixed(4)),             // 8E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 8F
            (MOS6502::bcc, AddressingMode::Relative, Cycles::Variable(2)),          // 90
            (MOS6502::sta, AddressingMode::IndirectYIndex, Cycles::Fixed(6)),       // 91
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 92
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 93
            (MOS6502::sty, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // 94
            (MOS6502::sta, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // 95
            (MOS6502::stx, AddressingMode::ZeropageYIndex, Cycles::Fixed(4)),       // 96
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 97
            (MOS6502::tya, AddressingMode::Implied, Cycles::Fixed(2)),              // 98
            (MOS6502::sta, AddressingMode::AbsoluteYIndex, Cycles::Fixed(5)),       // 99
            (MOS6502::txs, AddressingMode::Implied, Cycles::Fixed(2)),              // 9A
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 9B
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 9C
            (MOS6502::sta, AddressingMode::AbsoluteXIndex, Cycles::Fixed(5)),       // 9D
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 9E
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // 9F
            (MOS6502::ldy, AddressingMode::Immediate, Cycles::Fixed(2)),            // A0
            (MOS6502::lda, AddressingMode::XIndexIndirect, Cycles::Fixed(6)),       // A1
            (MOS6502::ldx, AddressingMode::Immediate, Cycles::Fixed(2)),            // A2
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // A3
            (MOS6502::ldy, AddressingMode::Zeropage, Cycles::Fixed(3)),             // A4
            (MOS6502::lda, AddressingMode::Zeropage, Cycles::Fixed(3)),             // A5
            (MOS6502::ldx, AddressingMode::Zeropage, Cycles::Fixed(3)),             // A6
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // A7
            (MOS6502::tay, AddressingMode::Implied, Cycles::Fixed(2)),              // A8
            (MOS6502::lda, AddressingMode::Immediate, Cycles::Fixed(2)),            // A9
            (MOS6502::tax, AddressingMode::Implied, Cycles::Fixed(2)),              // AA
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // AB
            (MOS6502::ldy, AddressingMode::Absolute, Cycles::Fixed(4)),             // AC
            (MOS6502::lda, AddressingMode::Absolute, Cycles::Fixed(4)),             // AD
            (MOS6502::ldx, AddressingMode::Absolute, Cycles::Fixed(4)),             // AE
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // AF
            (MOS6502::bcs, AddressingMode::Relative, Cycles::Variable(2)),          // B0
            (MOS6502::lda, AddressingMode::IndirectYIndex, Cycles::Variable(5)),    // B1
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // B2
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // B3
            (MOS6502::ldy, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // B4
            (MOS6502::lda, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // B5
            (MOS6502::ldx, AddressingMode::ZeropageYIndex, Cycles::Fixed(4)),       // B6
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // B7
            (MOS6502::clv, AddressingMode::Implied, Cycles::Fixed(2)),              // B8
            (MOS6502::lda, AddressingMode::AbsoluteYIndex, Cycles::Variable(4)),    // B9
            (MOS6502::tsx, AddressingMode::Implied, Cycles::Fixed(2)),              // BA
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // BB
            (MOS6502::ldy, AddressingMode::AbsoluteXIndex, Cycles::Variable(4)),    // BC
            (MOS6502::lda, AddressingMode::AbsoluteXIndex, Cycles::Variable(4)),    // BD
            (MOS6502::ldx, AddressingMode::AbsoluteYIndex, Cycles::Variable(4)),    // BE
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // BF
            (MOS6502::cpy, AddressingMode::Immediate, Cycles::Fixed(2)),            // C0
            (MOS6502::cmp, AddressingMode::XIndexIndirect, Cycles::Fixed(6)),       // C1
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // C2
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // C3
            (MOS6502::cpy, AddressingMode::Zeropage, Cycles::Fixed(3)),             // C4
            (MOS6502::cmp, AddressingMode::Zeropage, Cycles::Fixed(3)),             // C5
            (MOS6502::dec, AddressingMode::Zeropage, Cycles::Fixed(5)),             // C6
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // C7
            (MOS6502::iny, AddressingMode::Implied, Cycles::Fixed(2)),              // C8
            (MOS6502::cmp, AddressingMode::Immediate, Cycles::Fixed(2)),            // C9
            (MOS6502::dex, AddressingMode::Implied, Cycles::Fixed(2)),              // CA
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // CB
            (MOS6502::cpy, AddressingMode::Absolute, Cycles::Fixed(4)),             // CC
            (MOS6502::cmp, AddressingMode::Absolute, Cycles::Fixed(4)),             // CD
            (MOS6502::dec, AddressingMode::Absolute, Cycles::Fixed(6)),             // CE
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // CF
            (MOS6502::bne, AddressingMode::Relative, Cycles::Variable(2)),          // D0
            (MOS6502::cmp, AddressingMode::IndirectYIndex, Cycles::Variable(5)),    // D1
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // D2
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // D3
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // D4
            (MOS6502::cmp, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // D5
            (MOS6502::dec, AddressingMode::ZeropageXIndex, Cycles::Fixed(6)),       // D6
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // D7
            (MOS6502::cld, AddressingMode::Implied, Cycles::Fixed(2)),              // D8
            (MOS6502::cmp, AddressingMode::AbsoluteYIndex, Cycles::Variable(4)),    // D9
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // DA
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // DB
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // DC
            (MOS6502::cmp, AddressingMode::AbsoluteXIndex, Cycles::Variable(4)),    // DD
            (MOS6502::dec, AddressingMode::AbsoluteXIndex, Cycles::Fixed(7)),       // DE
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // DF
            (MOS6502::cpx, AddressingMode::Immediate, Cycles::Fixed(2)),            // E0
            (MOS6502::sbc, AddressingMode::XIndexIndirect, Cycles::Fixed(6)),       // E1
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // E2
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // E3
            (MOS6502::cpx, AddressingMode::Zeropage, Cycles::Fixed(3)),             // E4
            (MOS6502::sbc, AddressingMode::Zeropage, Cycles::Fixed(3)),             // E5
            (MOS6502::inc, AddressingMode::Zeropage, Cycles::Fixed(5)),             // E6
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // E7
            (MOS6502::inx, AddressingMode::Implied, Cycles::Fixed(2)),              // E8
            (MOS6502::sbc, AddressingMode::Immediate, Cycles::Fixed(2)),            // E9
            (MOS6502::nop, AddressingMode::Implied, Cycles::Fixed(2)),              // EA
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // EB
            (MOS6502::cpx, AddressingMode::Absolute, Cycles::Fixed(4)),             // EC
            (MOS6502::sbc, AddressingMode::Absolute, Cycles::Fixed(4)),             // ED
            (MOS6502::inc, AddressingMode::Absolute, Cycles::Fixed(6)),             // EE
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // EF
            (MOS6502::beq, AddressingMode::Relative, Cycles::Variable(2)),          // F0
            (MOS6502::sbc, AddressingMode::IndirectYIndex, Cycles::Variable(5)),    // F1
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // F2
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // F3
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // F4
            (MOS6502::sbc, AddressingMode::ZeropageXIndex, Cycles::Fixed(4)),       // F5
            (MOS6502::inc, AddressingMode::ZeropageXIndex, Cycles::Fixed(6)),       // F6
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // F7
            (MOS6502::sed, AddressingMode::Implied, Cycles::Fixed(2)),              // F8
            (MOS6502::sbc, AddressingMode::AbsoluteYIndex, Cycles::Variable(4)),    // F9
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // FA
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // FB
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // FC
            (MOS6502::sbc, AddressingMode::AbsoluteXIndex, Cycles::Variable(4)),    // FD
            (MOS6502::inc, AddressingMode::AbsoluteXIndex, Cycles::Fixed(7)),       // FE
            (MOS6502::not_implemented, AddressingMode::Implied, Cycles::Fixed(0)),  // FF
        ])
    }
}

impl<T: Bus> Default for MOS6502<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Bus> MOS6502<T> {
    /// Create new instance of MOS6502
    pub fn new() -> Self {
        Self {
            accumulator: u8::MIN,
            x_register: u8::MIN,
            y_register: u8::MIN,
            program_counter: u16::MIN,
            stack_pointer: u8::MAX,
            status_register: CpuFlags::Unused | CpuFlags::Break,
            opcode_array: OpcodeFunctionArray::default(),
        }
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

    #[inline]
    fn pop_from_stack(&mut self, bus: &mut T) -> Result<u8, BusError> {
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
    ) -> Result<u32, CpuError> {
        let (return_address_lo, return_address_hi): (u8, u8) = return_address.to_le_bytes().into();

        self.push_to_stack(bus, return_address_hi)?;
        self.push_to_stack(bus, return_address_lo)?;

        let (vector_address, status_register_value): (u16, CpuFlags) = match kind {
            InterruptKind::Irq => (0xFFFE, self.status_register & !CpuFlags::Break),
            InterruptKind::Nmi => (0xFFFA, self.status_register & !CpuFlags::Break),
            InterruptKind::Brk => (0xFFFE, self.status_register | CpuFlags::Break),
        };
        self.push_to_stack(bus, (status_register_value | CpuFlags::Unused).into())?;

        let divert_address_lo = bus.read(vector_address)?;
        let divert_address_hi = bus.read(vector_address + 1)?;

        self.set_program_counter(u16::from_le_bytes([divert_address_lo, divert_address_hi]));
        self.flag_set(CpuFlags::NoInterrupts, true);

        Ok(7)
    }

    pub fn irq(&mut self, bus: &mut T) -> Result<u32, CpuError> {
        if self.flag_check(CpuFlags::NoInterrupts) {
            return Ok(0);
        }
        self.perform_interrupt(self.program_counter, InterruptKind::Irq, bus)
    }

    pub fn nmi(&mut self, bus: &mut T) -> Result<u32, CpuError> {
        self.perform_interrupt(self.program_counter, InterruptKind::Nmi, bus)
    }

    fn not_implemented(&mut self, _: &mut impl Bus, _: AddressingMode) -> Result<u32, CpuError> {
        Err(CpuError::OpcodeNotImplemented)
    }

    /// Step over one CPU instruction
    pub fn step(&mut self, bus: &mut T) -> Result<u32, CpuError> {
        let opcode = bus.read(self.program_counter)? as usize;
        self.increment_program_counter(1);
        let (ref opcode_func, address_mode, base_cycles) = self.opcode_array.0[opcode];
        let spent_cycles = opcode_func(self, bus, address_mode)?;
        match base_cycles {
            Cycles::Fixed(n) => Ok(n),
            Cycles::Variable(n) => Ok(spent_cycles + n),
        }
    }

    /// Check if specified flag is set
    #[inline]
    pub fn flag_check(&self, flag: CpuFlags) -> bool {
        flag.intersects(self.status_register)
    }

    /// Turn specified flag on/off
    #[inline]
    fn flag_set(&mut self, f: CpuFlags, value: bool) {
        self.status_register.set(f, value);
    }

    #[inline]
    fn increment_program_counter(&mut self, n: u16) {
        self.set_program_counter(self.program_counter.wrapping_add(n));
    }

    /// Given some addressing mode, returns operand and increases CPU cycles as appropriate
    #[inline]
    fn resolve_operand(
        &mut self,
        bus: &mut T,
        address_mode: AddressingMode,
    ) -> Result<OpcodeOperand, CpuError> {
        match address_mode {
            AddressingMode::Accumulator => Ok(OpcodeOperand::Byte(self.accumulator)),
            AddressingMode::Absolute => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let address = u16::from_le_bytes([low_byte, high_byte]);

                Ok(OpcodeOperand::Address(address))
            }
            AddressingMode::AbsoluteXIndex => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let (low_byte, overflow) = low_byte.overflowing_add(self.x_register);
                high_byte = high_byte.wrapping_add(overflow as u8);

                let address = u16::from_le_bytes([low_byte, high_byte]);
                Ok(OpcodeOperand::AddressWithOverflow(address, overflow))
            }
            AddressingMode::AbsoluteYIndex => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let (low_byte, overflow) = low_byte.overflowing_add(self.y_register);
                high_byte = high_byte.wrapping_add(overflow as u8);

                let address = u16::from_le_bytes([low_byte, high_byte]);
                Ok(OpcodeOperand::AddressWithOverflow(address, overflow))
            }
            AddressingMode::Immediate => {
                let byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                Ok(OpcodeOperand::Byte(byte))
            }
            AddressingMode::Implied => Ok(OpcodeOperand::None),
            AddressingMode::Indirect => {
                let mut low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let address = u16::from_le_bytes([low_byte, high_byte]);

                low_byte = bus.read(address)?;
                high_byte = bus.read(address.wrapping_add(1))?;

                let operand = OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]));
                Ok(operand)
            }
            AddressingMode::XIndexIndirect => {
                let mut zeropage_address: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                zeropage_address = zeropage_address.wrapping_add(self.x_register);

                let low_byte = bus.read(zeropage_address as u16)?;
                let high_byte = bus.read(zeropage_address.wrapping_add(1) as u16)?;

                let operand = OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]));
                Ok(operand)
            }
            AddressingMode::IndirectYIndex => {
                let zeropage_address = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let low_byte = bus.read(zeropage_address as u16)?;
                let mut high_byte = bus.read(zeropage_address.wrapping_add(1) as u16)?;

                let (low_byte, overflow) = low_byte.overflowing_add(self.y_register);
                high_byte = high_byte.wrapping_add(overflow as u8);

                let operand = OpcodeOperand::AddressWithOverflow(
                    u16::from_le_bytes([low_byte, high_byte]),
                    overflow,
                );
                Ok(operand)
            }
            AddressingMode::Relative => {
                let offset = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let offset = (offset as i8) as i16;
                let new_program_counter = self.program_counter.wrapping_add_signed(offset);

                Ok(OpcodeOperand::Address(new_program_counter))
            }
            AddressingMode::Zeropage => {
                let zeropage_address = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                Ok(OpcodeOperand::Address(zeropage_address as u16))
            }
            AddressingMode::ZeropageXIndex => {
                let offset = self.x_register;

                let zeropage_address = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let address = zeropage_address.wrapping_add(offset);

                Ok(OpcodeOperand::Address(address as u16))
            }
            AddressingMode::ZeropageYIndex => {
                let offset = self.y_register;

                let zeropage_address = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let address = zeropage_address.wrapping_add(offset);

                Ok(OpcodeOperand::Address(address as u16))
            }
        }
    }
}
