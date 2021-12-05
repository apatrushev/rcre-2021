#![no_std]
#![no_main]

use stm32f3_discovery as bsp;
use bsp::stm32f3xx_hal as hal;

use panic_halt as _;
use usb_device;
use usbd_hid;

use cortex_m_rt::entry;
use hal::pac::{interrupt, CorePeripherals, Peripherals};
use hal::prelude::*;

use hal::usb;
use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_hid::descriptor::generator_prelude::*;
use usbd_hid::descriptor::MouseReport;
use usbd_hid::hid_class::HIDClass;

use cortex_m::asm::delay as cycle_delay;
use cortex_m::interrupt::free as disable_interrupts;
use cortex_m::peripheral::NVIC;
use cortex_m::asm::delay;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();

    // Configure clock to be USB compatible
    let mut flash = peripherals.FLASH.constrain();
    let mut rcc = peripherals.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz())
        .sysclk(48.MHz())
        .pclk1(24.MHz())
        .freeze(&mut flash.acr);
    assert!(clocks.usbclk_valid());
        
    let mut gpioa = peripherals.GPIOA.split(&mut rcc.ahb);
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
    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(usb::UsbBusType::new(usb::Peripheral {
            usb: peripherals.USB,
            pin_dm: usb_dm,
            pin_dp: usb_dp,
        }));
        USB_ALLOCATOR.as_ref().unwrap()
    };

    unsafe {
        USB_HID = Some(HIDClass::new(&bus_allocator, MouseReport::desc(), 60));
        USB_BUS = Some(
            UsbDeviceBuilder::new(&bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .manufacturer("Fake company")
                .product("Twitchy Mousey")
                .serial_number("TEST")
                .device_class(0xEF) // misc
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB_HP_CAN_TX, 1);
        core.NVIC.set_priority(interrupt::USB_LP_CAN_RX0, 1);
        NVIC::unmask(interrupt::USB_HP_CAN_TX);
        NVIC::unmask(interrupt::USB_LP_CAN_RX0);
    }

    loop {
        cycle_delay(25 * 1024 * 1024);
        push_mouse_movement(MouseReport {
            x: 0,
            y: 4,
            buttons: 0,
        })
        .ok()
        .unwrap_or(0);
        cycle_delay(25 * 1024 * 1024);
        push_mouse_movement(MouseReport {
            x: 0,
            y: -4,
            buttons: 0,
        })
        .ok()
        .unwrap_or(0);
    }
}

fn push_mouse_movement(report: MouseReport) -> Result<usize, usb_device::UsbError> {
    disable_interrupts(|_| unsafe { USB_HID.as_mut().map(|hid| hid.push_input(&report)) }).unwrap()
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<usb::UsbBusType>> = None;
static mut USB_BUS: Option<UsbDevice<usb::UsbBusType>> = None;
static mut USB_HID: Option<HIDClass<usb::UsbBusType>> = None;

fn poll_usb() {
    unsafe {
        USB_BUS.as_mut().map(|usb_dev| {
            USB_HID.as_mut().map(|hid| {
                usb_dev.poll(&mut [hid]);
            });
        });
    };
}

#[interrupt]
fn USB_HP_CAN_TX() {
    poll_usb();
}

#[interrupt]
fn USB_LP_CAN_RX0() {
    poll_usb();
}
