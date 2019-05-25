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

use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOB peripheral
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb4);

    // Configure gpio B pin 7 as a push-pull output.
    let mut ld2 = gpiob.pb7
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // Configure gpio B pin 14 as a push-pull output.
    let mut ld3 = gpiob.pb14
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    // Acquire the GPIOD peripheral
    let mut gpiod = dp.GPIOD.split(&mut rcc.ahb4);

    // initialize serial
    let tx = gpiod.pd8.into_af7(&mut gpiod.moder, &mut gpiod.afrh);
    let rx = gpiod.pd9.into_af7(&mut gpiod.moder, &mut gpiod.afrh);

    let mut serial = Serial::usart3(
        dp.USART3,
        (tx, rx),
        115_200.bps(),
        clocks,
        &mut rcc.apb1,
    );

    serial.listen(serial::Event::Rxne);
    // serial.listen(serial::Event::Txe); // TODO I am confused why this is not needed.
    let (mut tx, mut rx) = serial.split();

    loop {
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
        // cortex_m::asm::wfi();
    }
}
