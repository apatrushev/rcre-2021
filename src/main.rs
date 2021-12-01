#![no_std]
#![no_main]

use core::{
    panic::PanicInfo,
    sync::atomic::{AtomicBool, Ordering},
};
use stm32f3 as _;
use stm32f3_discovery::{
    button,
    button::interrupt::TriggerMode,
    leds::Leds,
    stm32f3xx_hal::{interrupt, pac, prelude::*},
    switch_hal::ToggleableOutputSwitch,
    wait_for_interrupt,
};

static USER_BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[interrupt]
fn EXTI0() {
    button::interrupt::clear();
    USER_BUTTON_PRESSED.store(true, Ordering::Relaxed);
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

    button::interrupt::enable(&dp.EXTI, &dp.SYSCFG, TriggerMode::Rising);

    loop {
        if USER_BUTTON_PRESSED.swap(false, Ordering::AcqRel) {
            leds.ld3.toggle().ok();
        }

        wait_for_interrupt();
    }
}
