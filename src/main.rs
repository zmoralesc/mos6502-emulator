mod memory;
mod mos6502;

use memory::RAM;
use mos6502::{Bus, MOS6502};

fn main() {
    const ACC_VAL: u8 = 0xfe;
    let zp_addr = 0xa1;
    let mut ram = RAM::new(1024 * 64);
    ram.write(0xfffc, 0xa5);
    ram.write(0xfffd, zp_addr);
    ram.write(zp_addr as u16, ACC_VAL);

    let mut cpu = MOS6502::new(ram);
    cpu.set_program_counter(0xfffc);
    cpu.run_for_cycles(2);
    assert_eq!(cpu.get_cycles(), 3);
    assert_eq!(cpu.get_accumulator(), ACC_VAL);
}
