const X86_64_SERIAL_PORT_BASE: usize = 0x3F8;

pub fn init() {
    crate::utils::serial_print(X86_64_SERIAL_PORT_BASE, "Rustaos booting on X86_64!\n");
}