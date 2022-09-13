mod memory;
mod mos6502;

use memory::RAM;
use mos6502::{Bus, MOS6502};

fn main() {
    let acc_val = 0xef;
    let mut ram = RAM::new(1024 * 64);
    ram.write(0xFFFC, 0xA5);
    ram.write(0xFFFD, 0x00);
    ram.write(0x00, acc_val);

    let mut cpu = MOS6502::new(ram);
    cpu.set_program_counter(0xFFFC);
    cpu.run_for_cycles(2);
    assert_eq!(cpu.get_cycles(), 3);
    assert_eq!(cpu.get_accumulator(), acc_val);
}
