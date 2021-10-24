//! Blinks an LED
//!
//! This assumes that LD2 (blue) is connected to pb7 and LD3 (red) is connected
//! to pb14. This assumption is true for the nucleo-h743zi board.

#![no_std]
#![no_main]

use panic_halt as _;

use stm32h7xx_hal::{
    prelude::*,
};

use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let rcc = rcc.use_hse(8.mhz()).bypass_hse();
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure gpio B pin 7 as a push-pull output.
    let mut ld2 = gpiob.pb7.into_push_pull_output();

    // Configure gpio B pin 14 as a push-pull output.
    let mut ld3 = gpiob.pb14.into_push_pull_output();
    ld3.set_high().unwrap();

    let one_second = ccdr.clocks.sys_ck().0;

    // Get the delay provider.
    // TODO: Unfortunately, this somehow does not work..
    // let mut delay = cp.SYST.delay(ccdr.clocks);

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        ld2.set_high().unwrap();
        //delay.delay_ms(2000u16);
        cortex_m::asm::delay(one_second);
        ld2.set_low().unwrap();
        //delay.delay_ms(2000u16);
        cortex_m::asm::delay(one_second);
    }
}
