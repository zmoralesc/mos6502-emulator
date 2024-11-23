# MOS 6502 Emulator

MOS 6502 processor emulator for learning purposes.

# What's done #
- All 151 legal opcodes working.
- Passes [Klaus Dormann's functional test](https://github.com/Klaus2m5/6502_65C02_functional_tests) with decimal mode disabled.
- NMIs and IRQs work as expected (also tested with Klaus Dormann's test suite).

# What's missing' #
- Decimal mode.
- All undocumented/illegal opcodes.

# How to use #
Include the library as a dependency:
```toml
[dependencies]
mos6502-emulator = { path = "../mos6502-emulator" }
```

To use, simply implement the Bus trait, then call the CPU's `step` method, passing your bus as a parameter:
```rust
use mos6502_emulator::{
    error::{BusError, CpuError},
    mos6502::{Bus, MOS6502},
};

// Define a type to represent your bus
pub struct SimpleBus([u8; 65536]);

impl SimpleBus {
    pub fn new() -> Self {
        SimpleBus([0; 65536])
    }
}

// Implement the provided Bus trait for your bus type
impl Bus for SimpleBus {
    fn read(&mut self, address: u16) -> Result<u8, BusError> {
        self.0
            .get(address as usize)
            .copied()
            .ok_or(BusError::InvalidRead(address))
    }

    fn write(&mut self, address: u16, value: u8) -> Result<(), BusError> {
        if let Some(byte) = self.0.get_mut(address as usize) {
            *byte = value;
            Ok(())
        } else {
            Err(BusError::InvalidWrite(address))
        }
    }
}

fn main() -> Result<(), CpuError> {
    let mut bus = SimpleBus::new();

    // Simple program that increments $0020 from 0 to 5
    let program = [
        0xA9, 0x00, // LDA #$00
        0x85, 0x20, // STA $0020
        0xA5, 0x20, // LDA $0020 (Loop start)
        0x18,       // CLC
        0x69, 0x01, // ADC #$01
        0x85, 0x20, // STA $0020
        0xC9, 0x05, // CMP #$05
        0xD0, 0xF7, // BNE Loop (Branch back to LDA $0020)
        0x00,       // BRK
    ];

    let start_address = 0x8000;
    for (i, &byte) in program.iter().enumerate() {
        bus.write(start_address + i as u16, byte)?;
    }

    // Create the CPU and set the program counter
    let mut cpu = MOS6502::new();
    cpu.set_program_counter(start_address);

    let mut total_cycles = 0;
    // Step through the program, stopping when the BRK instruction is reached
    while bus.read(cpu.program_counter())? != 0x00 {
        // If next opcode is ADC, read and display the value at $0020
        if bus.read(cpu.program_counter())? == 0x69 {
            let value_at_0020 = bus.read(0x0020)?;
            println!("Value at $0020: {:#04X}", value_at_0020);
        }
        total_cycles += cpu.step(&mut bus)?;
    }

    // Final value at $0020
    let final_value = bus.read(0x0020)?;
    println!("Final value at $0020: {:#04X}", final_value);
    println!("Total cycles: {}", total_cycles);

    Ok(())
}
```