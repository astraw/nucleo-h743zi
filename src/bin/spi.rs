//! SPI example application
//!
//! Connect a SPI device or a logic analyzer to the pins
//!  - D13: SPI_A_SCK
//!  - D12: SPI_A_MISO
//!  - D11: SPI_A_MOSI
//! The received bytes will be printed to the serial port via the ST-Link
#![no_main]
#![no_std]

use defmt_rtt as _; // global logger

use defmt::{debug, info};
use panic_probe as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{block, prelude::*, spi, timer};

#[entry]
fn main() -> ! {
    info!("Starting spi");

    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let rcc = rcc
        // .use_hse(8.mhz())
        // .bypass_hse()
        .sys_ck(400.MHz())
        .pll1_q_ck(100.MHz());
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOA peripheral. This also enables the clock for GPIOA in the RCC register
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    // See p.68 STM32Nucleo144 boards datasheet and p.90 STM32h743xI/G datasheet
    let sck = gpioa.pa5.into_alternate::<5>();
    let miso = gpioa.pa6.into_alternate();
    let mosi = gpioa.pa7.into_alternate();

    // Initialize the SPI peripheral.
    let mut spi = dp.SPI1.spi(
        (sck, miso, mosi),
        spi::MODE_0,
        1.MHz(),
        ccdr.peripheral.SPI1,
        &ccdr.clocks,
    );

    // Write fixed data
    spi.write(&[0x11u8, 0x22, 0x33]).unwrap();
    debug!("Wrote SPI bytes");

    // Configure the timer to trigger an update every second
    let mut timer = timer::Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.Hz());

    // Echo what is received on the SPI
    info!("Entering main loop");
    loop {
        let mut transfer_buf = [0x11u8, 0x22, 0x33];
        match spi.transfer(&mut transfer_buf) {
            Ok(read_bytes) => {
                // Log the received bytes. Currently this returns bytes (0x00)
                // with no error even when no SPI is transmitting. TODO: figure
                // out why and fix it.
                debug!("Received SPI bytes: {:?}\r", read_bytes);
            }
            Err(e) => {
                panic!("SPI transfer error occurred: {:?}", e);
            }
        }
        block!(timer.wait()).unwrap();
    }
}
