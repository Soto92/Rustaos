const RISCV64_SERIAL_PORT_BASE: usize = 0x10000000;

pub fn init() {
    crate::utils::serial_print(RISCV64_SERIAL_PORT_BASE, "Rustaos booting on RISCV64!\n");
}