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

type OperandResult = Result<(u32, OpcodeOperand), CpuError>;
type OpcodeFunction<T> = fn(&mut MOS6502<T>, &mut T, AddressingMode) -> Result<u32, CpuError>;
struct OpcodeFunctionArray<T: Bus>([[OpcodeFunction<T>; 8]; 4]);

enum OpcodeOperand {
    Byte(u8),
    Address(u16),
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
            // group three (00)
            [
                MOS6502::not_implemented,
                MOS6502::bit,
                MOS6502::jmp,
                MOS6502::jmp,
                MOS6502::sty,
                MOS6502::ldy,
                MOS6502::cpy,
                MOS6502::cpx,
            ],
            // group one (01)
            [
                MOS6502::ora,
                MOS6502::and,
                MOS6502::eor,
                MOS6502::adc,
                MOS6502::sta,
                MOS6502::lda,
                MOS6502::cmp,
                MOS6502::sbc,
            ],
            // group two (10)
            [
                MOS6502::asl,
                MOS6502::rol,
                MOS6502::lsr,
                MOS6502::ror,
                MOS6502::stx,
                MOS6502::ldx,
                MOS6502::dec,
                MOS6502::inc,
            ],
            // group four (11)
            [
                MOS6502::not_implemented,
                MOS6502::not_implemented,
                MOS6502::not_implemented,
                MOS6502::not_implemented,
                MOS6502::not_implemented,
                MOS6502::not_implemented,
                MOS6502::not_implemented,
                MOS6502::not_implemented,
            ],
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

        let opcode_number: usize = (opcode >> 5) as usize;
        let addressing_mode: usize = ((opcode & 0b00011100) >> 2) as usize;
        let opcode_group: usize = (opcode & 0b00000011) as usize;

        self.increment_program_counter(1);

        let ref opcode_func = self.opcode_array.0[opcode_group][opcode_number];
        let address_mode = todo!();
        opcode_func(self, bus, address_mode)
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
    fn resolve_operand(&mut self, bus: &mut T, address_mode: AddressingMode) -> OperandResult {
        match address_mode {
            AddressingMode::Accumulator => Ok((1, OpcodeOperand::Byte(self.accumulator))),
            AddressingMode::Absolute => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let address = u16::from_le_bytes([low_byte, high_byte]);

                Ok((3, OpcodeOperand::Address(address)))
            }
            AddressingMode::AbsoluteXIndex => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let (low_byte, overflow) = low_byte.overflowing_add(self.x_register);
                high_byte = high_byte.wrapping_add(overflow as u8);

                let address = u16::from_le_bytes([low_byte, high_byte]);
                Ok((3 + overflow as u32, OpcodeOperand::Address(address)))
            }
            AddressingMode::AbsoluteYIndex => {
                let low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let (low_byte, overflow) = low_byte.overflowing_add(self.y_register);
                high_byte = high_byte.wrapping_add(overflow as u8);

                let cycles = 3 + overflow as u32;
                let address = u16::from_le_bytes([low_byte, high_byte]);
                Ok((cycles, OpcodeOperand::Address(address)))
            }
            AddressingMode::Immediate => {
                let byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                Ok((1, OpcodeOperand::Byte(byte)))
            }
            AddressingMode::Implied => Ok((0, OpcodeOperand::None)),
            AddressingMode::Indirect => {
                let mut low_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);
                let mut high_byte: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let address = u16::from_le_bytes([low_byte, high_byte]);

                low_byte = bus.read(address)?;
                high_byte = bus.read(address.wrapping_add(1))?;

                let operand = OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]));
                Ok((2, operand))
            }
            AddressingMode::XIndexIndirect => {
                let mut zeropage_address: u8 = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                zeropage_address = zeropage_address.wrapping_add(self.x_register);

                let low_byte = bus.read(zeropage_address as u16)?;
                let high_byte = bus.read(zeropage_address.wrapping_add(1) as u16)?;

                let operand = OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]));
                Ok((5, operand))
            }
            AddressingMode::IndirectYIndex => {
                let zeropage_address = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let low_byte = bus.read(zeropage_address as u16)?;
                let mut high_byte = bus.read(zeropage_address.wrapping_add(1) as u16)?;

                let (low_byte, overflow) = low_byte.overflowing_add(self.y_register);
                high_byte = high_byte.wrapping_add(overflow as u8);

                let operand = OpcodeOperand::Address(u16::from_le_bytes([low_byte, high_byte]));
                Ok((4 + overflow as u32, operand))
            }
            AddressingMode::Relative => {
                let offset = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let current_page = self.program_counter >> 8;

                let offset = (offset as i8) as i16;
                let new_program_counter = self.program_counter.wrapping_add_signed(offset);

                let page_transition_ocurred = new_program_counter >> 8 != current_page;

                let cycles = 1 + page_transition_ocurred as u32;
                Ok((cycles, OpcodeOperand::Address(new_program_counter)))
            }
            AddressingMode::Zeropage => {
                let zeropage_address = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                Ok((2, OpcodeOperand::Address(zeropage_address as u16)))
            }
            AddressingMode::ZeropageXIndex => {
                let offset = self.x_register;

                let zeropage_address = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let address = zeropage_address.wrapping_add(offset);

                Ok((3, OpcodeOperand::Address(address as u16)))
            }
            AddressingMode::ZeropageYIndex => {
                let offset = self.y_register;

                let zeropage_address = bus.read(self.program_counter)?;
                self.increment_program_counter(1);

                let address = zeropage_address.wrapping_add(offset);

                Ok((3, OpcodeOperand::Address(address as u16)))
            }
        }
    }
}
