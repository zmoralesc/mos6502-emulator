mod mos6502;

use mos6502::{MOS6502, RAM};

fn main() {
    let mut mem = RAM::new(1024 * 64);
    let mut cpu = MOS6502::new(&mut mem);
    cpu.flag_check(mos6502::FLAG_CARRY);
    println!("Hello, world!");
}
