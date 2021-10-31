//! SPI example application
//!
//! Connect a SPI device or a logic analyzer to the pins
//!  - D13: SPI_A_SCK
//!  - D12: SPI_A_MISO
//!  - D11: SPI_A_MOSI
//! The received bytes will be printed to the serial port via the ST-Link
#![no_main]
#![no_std]

use panic_halt as _;

use cortex_m_rt::entry;
use stm32h7xx_hal::{prelude::*, timer, spi, block, serial};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let dp = stm32h7xx_hal::stm32::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let rcc = rcc.use_hse(8.mhz()).bypass_hse().sys_ck(400.mhz()).pll1_q_ck(100.mhz());
    let ccdr = rcc.freeze(pwrcfg, &dp.SYSCFG);

    // Acquire the GPIOC peripheral. This also enables the clock for GPIOA in the RCC register
    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);

    // See p.68 STM32Nucleo144 boards datasheet and p.90 STM32h743xI/G datasheet
    let sck = gpioa.pa5.into_alternate_af5();
    let miso = gpioa.pa6.into_alternate_af5();
    let mosi = gpioa.pa7.into_alternate_af5();

    // Initialise the SPI peripheral.
    let mut spi = dp.SPI1.spi(
        (sck, miso, mosi),
        spi::MODE_0,
        1.mhz(),
        ccdr.peripheral.SPI1,
        &ccdr.clocks,
    );

    // Acquire the GPIOD peripheral
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    // initialize serial
    gpiod.pd8.into_alternate_af7();
    gpiod.pd9.into_alternate_af7();

    let serial = serial::Serial::usart3(
        dp.USART3,
        serial::config::Config::default().baudrate(115200.bps()),
        ccdr.peripheral.USART3,
        &ccdr.clocks
    ).unwrap();
    let (mut tx, _rx) = serial.split();

    // Write fixed data
    spi.write(&[0x11u8, 0x22, 0x33]).unwrap();

    // Configure the timer to trigger an update every second
    let mut timer = timer::Timer::tim1(dp.TIM1, ccdr.peripheral.TIM1, &ccdr.clocks);
    timer.start(1.hz());

    // Echo what is received on the SPI
    //let mut received = 0;
    loop {
        let mut transfer_buf = [0x11u8, 0x22, 0x33];
        match spi.transfer(&mut transfer_buf) {
            Ok(read_bytes) => {
                // log the received bytes via the serial port here
                writeln!(tx, "Received SPI bytes: {:?}\r", read_bytes).unwrap();
            }
            Err(_) => {
                panic!("SPI transfer error occured")
            }
        }
        block!(timer.wait()).unwrap();
    }
}
