//! Blinks an LED
//!
//! This assumes that LD1 (green) is connected to pb0 and LD3 (red) is connected
//! to pb14. This assumption is true for the nucleo-h743zi and nucleo-h743zi2
//! boards.

#![no_std]
#![no_main]

use defmt_rtt as _; // global logger

use defmt::info;
use panic_probe as _;

use stm32h7xx_hal::{block, prelude::*, timer::Timer};

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    info!("Starting blinky");

    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOB peripheral
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    // Configure gpio B pin 0 as a push-pull output.
    let mut ld1 = gpiob.pb0.into_push_pull_output();

    // Configure gpio B pin 14 as a push-pull output.
    let mut ld3 = gpiob.pb14.into_push_pull_output();
    ld3.set_high();

    // Configure the timer to trigger an update every second
    let mut timer = Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.Hz());

    // Wait for the timer to trigger an update and change the state of the LED
    info!("Entering main loop");
    loop {
        ld1.set_high();
        block!(timer.wait()).unwrap();
        ld1.set_low();
        block!(timer.wait()).unwrap();
    }
}
