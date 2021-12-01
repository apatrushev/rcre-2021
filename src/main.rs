#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate rtic;

use core::sync::atomic::{self, Ordering};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f3_discovery::{
    button,
    button::interrupt::TriggerMode,
    leds::Leds,
    stm32f3xx_hal::prelude::*,
    switch_hal::ToggleableOutputSwitch,
};

#[rtic::app(device = stm32f3_discovery::stm32f3xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        leds: Leds,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("Starting");

        let mut rcc = ctx.device.RCC.constrain();
        let mut gpioe = ctx.device.GPIOE.split(&mut rcc.ahb);

        let leds = Leds::new(
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

        button::interrupt::enable(
            &ctx.device.EXTI,
            &ctx.device.SYSCFG,
            TriggerMode::Rising,
        );

        rprintln!("Started");
        init::LateResources { leds }
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {
            atomic::compiler_fence(Ordering::SeqCst);
        }
    }

    #[task(binds = EXTI0, resources = [leds])]
    fn button_click(ctx: button_click::Context) {
        rprintln!("Click");
        button::interrupt::clear();
        ctx.resources.leds.ld3.toggle().ok();
    }
};
