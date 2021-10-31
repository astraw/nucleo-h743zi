//! Blinks an LED
//!
//! This assumes that LD1 (green) is connected to pb0, LD2 (yellow) is connected
//! to pe1 and LD3 (red) is connected to pb14. This assumption is true for the
//! nucleo-h743zi2 board.

#![no_std]
#![no_main]

use panic_halt as _;

use stm32h7xx_hal::{
    prelude::*,
    timer::Timer,
    block
};

use cortex_m_rt::entry;

use embedded_hal::digital::v2::OutputPin;

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    let rcc = rcc.use_hse(8.mhz()).bypass_hse();
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure gpio B pin 0 as a push-pull output.
    let mut ld1 = gpiob.pb0.into_push_pull_output();

    // Acquire the GPIOE peripheral
    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);

    // Configure gpio E pin 1 as a push-pull output.
    let mut ld2 = gpioe.pe1.into_push_pull_output();

    // Configure gpio B pin 14 as a push-pull output.
    let mut ld3 = gpiob.pb14.into_push_pull_output();
    ld3.set_high().unwrap();

    // Configure the timer to trigger an update every second
    let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.hz());

    let mut count: u8 = 0;

    // Count from 0 to 8 and show the counter in binary bits in the LEDs.
    loop {

        if (count & 0x01) != 0 {
            ld1.set_high().unwrap();
        } else {
            ld1.set_low().unwrap();
        }

        if (count & 0x02) != 0 {
            ld2.set_high().unwrap();
        } else {
            ld2.set_low().unwrap();
        }

        if (count & 0x04) != 0 {
            ld3.set_high().unwrap();
        } else {
            ld3.set_low().unwrap();
        }

        block!(timer.wait()).unwrap();

        count += 1;
        if count >= 8 {
            count = 0;
        }
    }
}
