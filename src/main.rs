#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use stm32f3;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}
