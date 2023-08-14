#![allow(unused)]

mod composite_bus;
mod memory;
mod mos6502;

use memory::Ram;
use mos6502::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ldx_immediate() {
        let program_start = 0xfffc;
        let x_value: u8 = 0xee;
        let cycles_to_run = 2;

        let ram = Ram::new(1024 * 64);
        let mut cpu = MOS6502::new(ram, false, None);

        cpu.write_to_bus(program_start, 0xa2);
        cpu.write_to_bus(program_start + 1, x_value);

        cpu.set_program_counter(program_start);
        cpu.run_for_cycles(cycles_to_run);
        assert_eq!(cpu.cycles(), cycles_to_run);
        assert_eq!(cpu.x_register(), x_value);
    }

    #[test]
    fn test_lda_zeropage_x_index() {
        let program_start: u16 = 0xfffc;
        let acc_value: u8 = 0xee;
        let zp_addr: u8 = 0xa1;
        let offset: u8 = 0x02;
        let cycles_to_run = 6; // 2 for LDX, 4 for LDA

        let ram = Ram::new(1024 * 64);
        let mut cpu = MOS6502::new(ram, false, None);

        let program = vec![
            0xa2,    // LDX, immediate
            offset,  // Offset
            0xb5,    // LDA, Zeropage X-indexed
            zp_addr, // Zeropage address
        ];

        for (i, byte) in program.iter().enumerate() {
            cpu.write_to_bus(program_start + i as u16, *byte);
        }
        cpu.write_to_bus(zp_addr.wrapping_add(offset) as u16, acc_value);

        cpu.set_program_counter(program_start);
        cpu.run_for_cycles(cycles_to_run);
        assert_eq!(cpu.cycles(), cycles_to_run);
        assert_eq!(cpu.accumulator(), acc_value);
        assert_eq!(cpu.x_register(), offset);
    }

    #[test]
    fn test_adc() {
        let program_start: u16 = 0xff0c;
        let cycles_to_run = 6;

        let ram = Ram::new(1024 * 64);
        let mut cpu = MOS6502::new(ram, false, None);

        let value_to_store: u8 = 0xe5;
        let value_to_add: u8 = 0x01;

        let program = vec![
            0xa2,           // LDX, immediate
            value_to_store, // E5
            0x8a,           // TXA
            0x69,           // ADC, immediate
            value_to_add,   // Value to add
        ];

        for (i, byte) in program.iter().enumerate() {
            cpu.write_to_bus(program_start + i as u16, *byte);
        }

        cpu.set_program_counter(program_start);
        cpu.run_for_cycles(cycles_to_run);
        assert_eq!(cpu.cycles(), cycles_to_run);
        assert_eq!(cpu.accumulator(), value_to_store.wrapping_add(value_to_add));
    }

    #[test]
    fn test_sbc() {
        let program_start: u16 = 0xff0c;
        let cycles_to_run = 6;

        let ram = Ram::new(1024 * 64);
        let mut cpu = MOS6502::new(ram, false, None);

        let value_to_store: u8 = 0xe5;
        let value_to_subtract: u8 = 0x02;

        let program = vec![
            0xa2,              // LDX, immediate
            value_to_store,    // E5
            0x8a,              // TXA
            0xe9,              // SBC, immediate
            value_to_subtract, // Value to subtract
        ];

        for (i, byte) in program.iter().enumerate() {
            cpu.write_to_bus(program_start + i as u16, *byte);
        }

        cpu.set_program_counter(program_start);
        cpu.run_for_cycles(cycles_to_run);
        assert_eq!(cpu.cycles(), cycles_to_run);
        assert_eq!(
            cpu.accumulator(),
            value_to_store.wrapping_sub(value_to_subtract)
        );
    }
}
