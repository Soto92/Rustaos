const AARCH64_SERIAL_PORT_BASE: usize = 0x09000000;

pub fn init() {
    crate::utils::serial_print(AARCH64_SERIAL_PORT_BASE, "Rustaos booting on aarch64!\n");
}