//! Echo bytes over serial
//!
//! This assumes that serial TX is PD8 and RX is PD9. This is true for the
//! nucleo-h743zi board in which these are connected to the ST-LINK virtual COM
//! port. Furthermore, pb7 is used as LD2 and pb14 is used as LD3.

#![no_std]
#![no_main]

use panic_halt as _;

use stm32h7xx_hal::{
    prelude::*,
    serial::{self, Serial, Error},
};
use core::fmt::Write;

#[macro_use(block)]
extern crate nb;

use embedded_hal::digital::v2::OutputPin;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let rcc = rcc.sys_ck(400.mhz()).use_hse(8.mhz()).bypass_hse();
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure gpio B pin 7 as a push-pull output.
    let mut ld2 = gpiob.pb7.into_push_pull_output();
    // Configure gpio B pin 14 as a push-pull output.
    let mut ld3 = gpiob.pb14.into_push_pull_output();

    // Acquire the GPIOD peripheral
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    // initialize serial
    gpiod.pd8.into_alternate_af7();
    gpiod.pd9.into_alternate_af7();

    let serial = Serial::usart3(
        dp.USART3,
        serial::config::Config::default().baudrate(115200.bps()),
        ccdr.peripheral.USART3,
        &ccdr.clocks
    ).unwrap();

    let (mut tx, mut rx) = serial.split();

    // core::fmt::Write is implemented for tx.
    writeln!(tx, "Hello World\r").unwrap();
    writeln!(tx, "Entering echo mode..\r").unwrap();
    loop {
        // Echo what is received on the serial link.
        match rx.read() {
            Ok(c) => {
                tx.write(c).unwrap();
            }
            Err(nb::Error::WouldBlock) => {
            },
            Err(nb::Error::Other(err)) => {
                match err {
                    Error::Framing => {
                        ld2.set_high().unwrap(); // blue
                    }
                    _ => {
                        ld3.set_high().unwrap(); // red
                    }
                }
                panic!("");
            },
        }
        let received = block!(rx.read()).unwrap();
        block!(tx.write(received)).ok();
    }
}
