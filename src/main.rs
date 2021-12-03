#![deny(warnings)]
#![no_std]
#![no_main]

extern crate rtic;

use core::sync::atomic::{self, Ordering};
use cortex_m::asm::delay;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f3_discovery::{
    button,
    button::interrupt::TriggerMode,
    leds::Leds,
    stm32f3xx_hal::{prelude::*, usb},
    switch_hal::ToggleableOutputSwitch,
};
use usb_device::{bus, prelude::*};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[rtic::app(device = stm32f3_discovery::stm32f3xx_hal::pac, peripherals = true)]
const APP: () = {
    struct Resources {
        usb_dev: UsbDevice<'static, usb::UsbBusType>,
        serial: SerialPort<'static, usb::UsbBusType>,
        leds: Leds,
    }

    #[init]
    fn init(ctx: init::Context) -> init::LateResources {
        rtt_init_print!();
        rprintln!("Starting");

        static mut USB_BUS: Option<bus::UsbBusAllocator<usb::UsbBusType>> =
            None;

        // Configure clock to be USB compatible
        let mut flash = ctx.device.FLASH.constrain();
        let mut rcc = ctx.device.RCC.constrain();
        let clocks = rcc
            .cfgr
            .use_hse(8.MHz())
            .sysclk(48.MHz())
            .pclk1(24.MHz())
            .freeze(&mut flash.acr);
        assert!(clocks.usbclk_valid());

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

        // USB configuration
        {
            let mut gpioa = ctx.device.GPIOA.split(&mut rcc.ahb);
            // F3 Discovery board has a pull-up resistor on the D+ line.
            // Pull the D+ pin down to send a RESET condition to the USB bus.
            // This forced reset is needed only for development, without it host
            // will not reset your device when you upload new firmware.
            let mut usb_dp = gpioa
                .pa12
                .into_push_pull_output(&mut gpioa.moder, &mut gpioa.otyper);
            usb_dp.set_low().ok();
            delay(clocks.sysclk().0 / 100);

            let usb_dm = gpioa.pa11.into_af14_push_pull(
                &mut gpioa.moder,
                &mut gpioa.otyper,
                &mut gpioa.afrh,
            );
            let usb_dp = usb_dp.into_af14_push_pull(
                &mut gpioa.moder,
                &mut gpioa.otyper,
                &mut gpioa.afrh,
            );

            let usb = usb::Peripheral {
                usb: ctx.device.USB,
                pin_dm: usb_dm,
                pin_dp: usb_dp,
            };
            unsafe { USB_BUS.replace(usb::UsbBus::new(usb)) };
        }

        // USB CDC configuration
        let serial = SerialPort::new(unsafe { USB_BUS.as_ref() }.unwrap());
        let usb_dev = {
            UsbDeviceBuilder::new(
                unsafe { USB_BUS.as_ref() }.unwrap(),
                UsbVidPid(0x16c0, 0x27dd),
            )
            .manufacturer("Fake company")
            .product("Serial port")
            .serial_number("TEST")
            .device_class(USB_CLASS_CDC)
            .build()
        };

        rprintln!("Started");
        init::LateResources {
            usb_dev,
            serial,
            leds,
        }
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

    #[task(binds = USB_HP_CAN_TX, resources = [usb_dev, serial, leds])]
    fn usb_tx(mut cx: usb_tx::Context) {
        usb_poll(&mut cx.resources.usb_dev, &mut cx.resources.serial, cx.resources.leds);
    }

    #[task(binds = USB_LP_CAN_RX0, resources = [usb_dev, serial, leds])]
    fn usb_rx0(mut cx: usb_rx0::Context) {
        usb_poll(&mut cx.resources.usb_dev, &mut cx.resources.serial, cx.resources.leds);
    }
};

fn usb_poll<B: bus::UsbBus>(
    usb_dev: &mut UsbDevice<'static, B>,
    serial: &mut SerialPort<'static, B>,
    leds: &mut Leds,
) {
    if !usb_dev.poll(&mut [serial]) {
        return;
    }

    let mut buf = [0u8; 8];
    match serial.read(&mut buf) {
        Ok(count) if count > 0 => {
            // Echo back in upper case
            leds.ld4.toggle().ok();
            for c in buf[0..count].iter_mut() {
                if 0x61 <= *c && *c <= 0x7a {
                    *c &= !0x20;
                }
            }

            serial.write(&buf[0..count]).ok();
        }
        _ => {}
    }
}
