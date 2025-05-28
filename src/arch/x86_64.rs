pub fn init() {
    crate::arch::x86_64::serial_print("Rustaos booting on X86_64!\n");
}

pub fn serial_print(s: &str) {
    unsafe {
        for byte in s.bytes() {
            core::ptr::write_volatile(0x3F8 as *mut u8, byte);
        }
    }
}