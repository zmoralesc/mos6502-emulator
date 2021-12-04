mod memory;
mod mos6502;

use memory::{Bus, RAM};
use mos6502::MOS6502;

fn main() {
    let mut ram = RAM::new(1024 * 64);
    ram.bus_write(0xFFFC, 0xA9);
    ram.bus_write(0xFFFD, 0xA1);

    let mut cpu = MOS6502::new(&mut ram);
    cpu.set_pc(0xFFFC);
    cpu.run_for_cycles(2);
    assert_eq!(cpu.get_cycles(), 2);
}
