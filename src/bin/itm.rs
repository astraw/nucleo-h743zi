//! Shows the ITM functionality to get logger output
//!
//! `itmdump` is used to display the outoput.
//! The README contains information on additional tools required.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::iprintln;
use panic_halt as _;

use nucleo_h743zi::logging;
use log::{info, warn};

use stm32h7xx_hal::{block, prelude::*, timer};

#[entry]
fn main() -> ! {
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    // Take ownership over the RCC devices and convert them into the corresponding HAL structs
    let rcc = dp.RCC.constrain();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Freeze the configuration of all the clocks in the system and
    // retrieve the Core Clock Distribution and Reset (CCDR) object
    // Configure PLL1 R for TRACEC as well
    let rcc = rcc.sys_ck(400.mhz()).use_hse(8.mhz()).bypass_hse().pll1_r_ck(400.mhz());
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Configure the timer to trigger an update every second
    let mut timer = timer::Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.hz());

    unsafe {
        logging::itm::enable_itm(
            &dp.DBGMCU, &mut cp.DCB, &mut cp.ITM, &ccdr.clocks, 2000000
        );
    }
    // Set up ITM and more complex logger
    logging::itm::init();

    let stim = &mut cp.ITM.stim[0];
    iprintln!(stim, "Hello World");
    let mut counter = 0;

    loop {
        iprintln!(stim, "{}: This is a very useful logger", counter);
        info!("This is a more complex logger using the log framework");
        warn!("Warnings are also possible.");
        counter += 1;
        // Echo what is received on the serial link.
        block!(timer.wait()).unwrap();
    }
}
