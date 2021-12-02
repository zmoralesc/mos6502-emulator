mod mos6502;

use mos6502::MOS6502;

fn main() {
    let mut cpu = MOS6502::new();
    cpu.flag_check(mos6502::FLAG_CARRY);
    println!("Hello, world!");
}
