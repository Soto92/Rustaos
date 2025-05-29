#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod arch;
mod utils;

#[cfg(target_arch = "x86_64")]
use arch::x86_64 as arch_impl;

#[cfg(target_arch = "riscv64")]
use arch::riscv64 as arch_impl;

#[cfg(target_arch = "aarch64")]
use arch::aarch64 as arch_impl;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    arch_impl::init();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}