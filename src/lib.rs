#![allow(unused)]

mod memory;
mod mos6502;

use memory::Ram;
use mos6502::{Bus, MOS6502};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldx_immediate() {
        let program_start = 0xfffc;
        let x_value: u8 = 0xee;
        let cycles_to_run = 2;

        let ram = Ram::new(1024 * 64);
        let mut cpu = MOS6502::new(ram);

        let bus: &mut dyn Bus = cpu.bus();

        bus.write(program_start, 0xa2);
        bus.write(program_start + 1, x_value);

        cpu.set_program_counter(program_start);
        cpu.run_for_cycles(cycles_to_run);
        assert_eq!(cpu.cycles(), cycles_to_run);
        assert_eq!(cpu.x_register(), x_value);
    }

    #[test]
    fn test_lda_zeropage_x_index() {
        let program_start = 0xfffc;
        let acc_value: u8 = 0xee;
        let zp_addr = 0xa1;
        let offset = 0x02;
        let cycles_to_run = 4;

        let ram = Ram::new(1024 * 64);
        let mut cpu = MOS6502::new(ram);

        let bus: &mut dyn Bus = cpu.bus();

        bus.write(program_start, 0xa2);
        bus.write(program_start + 1, offset);

        bus.write(zp_addr as u16 + offset as u16, acc_value);

        bus.write(program_start + 2, 0xb5);
        bus.write(program_start + 3, zp_addr);

        cpu.set_program_counter(program_start);
        cpu.run_for_cycles(cycles_to_run);
        assert_eq!(cpu.cycles(), cycles_to_run);
        assert_eq!(cpu.accumulator(), acc_value);
        assert_eq!(cpu.x_register(), offset);
    }
}
