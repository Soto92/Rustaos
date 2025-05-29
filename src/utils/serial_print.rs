pub fn serial_print(port: usize, s: &str) {
    unsafe {
        for byte in s.bytes() {
            core::ptr::write_volatile(port as *mut u8, byte);
        }
    }
}
