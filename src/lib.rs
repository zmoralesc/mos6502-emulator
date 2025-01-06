pub mod error;
pub mod mos6502;

#[cfg(test)]
mod tests {
    use crate::error;
    use crate::mos6502;
    use mos6502::*;

    struct TestBus([u8; 65536]);

    impl Bus for TestBus {
        fn read(&mut self, address: u16) -> Result<u8, error::BusError> {
            Ok(self.0[address as usize])
        }

        fn write(&mut self, address: u16, value: u8) -> Result<(), error::BusError> {
            self.0[address as usize] = value;
            Ok(())
        }
    }

    #[test]
    fn test_6502_functional() {
        let test_bin_path = "6502_65C02_functional_tests/6502_functional_test.bin";
        let test_data = std::fs::read(test_bin_path).expect("Failed to load test suite");

        let mut ram = TestBus([0; 65536]);
        for i in 0..test_data.len() {
            ram.write(i as u16, test_data[i])
                .expect("Failed to write test binary to RAM");
        }

        let mut cpu = MOS6502::new();
        cpu.set_program_counter(0x400);

        let mut last_pc: u16 = 0;

        while cpu.program_counter() != last_pc {
            last_pc = cpu.program_counter();
            cpu.step(&mut ram).expect("Failed to step CPU");
        }
        assert_eq!(last_pc, 0x336d);
    }

    #[test]
    fn test_6502_interrupt() {
        let test_bin_path = "6502_65C02_functional_tests/6502_interrupt_test.bin";
        let test_data = std::fs::read(test_bin_path).expect("Failed to load test suite");

        let mut ram = TestBus([0; 65536]);
        for i in 0..test_data.len() {
            ram.write(i as u16, test_data[i])
                .expect("Failed to write test binary to RAM");
        }

        let mut cpu = MOS6502::new();

        let mut last_pc: u16 = 0;

        let feedback_port_address: u16 = 0xbffc;
        let mut current_i_port;
        let mut previous_i_port: u8 = ram
            .read(feedback_port_address)
            .expect("Failed to read feedback port");
        cpu.set_program_counter(0x400);
        while cpu.program_counter() != last_pc {
            last_pc = cpu.program_counter();

            current_i_port = ram
                .read(feedback_port_address)
                .expect("Failed to read feedback port");

            let nmi_triggered = previous_i_port & (1 << 1) != 0 && current_i_port & (1 << 1) == 0;
            let irq_triggered = current_i_port & (1 << 0) == 0;

            if nmi_triggered {
                cpu.nmi(&mut ram).expect("Failed to perform NMI");
            } else if irq_triggered {
                cpu.irq(&mut ram).expect("Failed to perform IRQ");
            }

            cpu.step(&mut ram).expect("Failed to step CPU");

            previous_i_port = current_i_port;
        }
        assert_eq!(last_pc, 0x06e5);
    }
}
