#![no_main]
#![no_std]

extern crate stm32h7xx_hal as stm32_hal;

use defmt_rtt as _; // global logger
use panic_probe as _;

#[rtic::app(device = stm32_hal::stm32, peripherals = true)]
mod app {
    use defmt::info;

    // Late resources
    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        info!("init");
        (Shared {}, Local {}, init::Monotonics())
    }

    #[idle(shared = [])]
    fn idle(_: idle::Context) -> ! {
        info!("idle");
        loop {}
    }
}
