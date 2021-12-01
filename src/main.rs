#![no_std]
#![no_main]
#![allow(unused_imports)]

use core::panic::PanicInfo;
use stm32f3;
use stm32f3_discovery::{
    leds::Leds,
    stm32f3xx_hal::{pac, prelude::*},
    switch_hal::ToggleableOutputSwitch,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.constrain();

    let mut gpioe = dp.GPIOE.split(&mut rcc.ahb);
    let mut leds = Leds::new(
        gpioe.pe8,
        gpioe.pe9,
        gpioe.pe10,
        gpioe.pe11,
        gpioe.pe12,
        gpioe.pe13,
        gpioe.pe14,
        gpioe.pe15,
        &mut gpioe.moder,
        &mut gpioe.otyper,
    );

    loop {
        leds.ld3.toggle().ok();
        cortex_m::asm::delay(1_000_000);
    }
}
