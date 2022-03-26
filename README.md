[![build](https://github.com/astraw/nucleo-h743zi/actions/workflows/cargo.yml/badge.svg)](https://github.com/astraw/nucleo-h743zi/actions/workflows/cargo.yml)

# nucleo-h743zi

Example programs for the [NUCLEO-H743ZI and
NUCLEO-H743ZI2](https://www.st.com/en/evaluation-tools/nucleo-h743zi.html)
boards.

This uses the [stm32h7xx-hal](https://github.com/stm32-rs/stm32h7xx-hal) crate.

Collaboration on this crate is highly welcome as are pull requests!

## Project Aim: Facilitate First Steps with Nucleo H743ZI and H743ZI2 Boards and Rust

This repository contains examples and starter material specific to the Nucleo
H743ZI and H743ZI2 boards and should work "out of the box" with these specific
boards and demonstrates the usage of features on these boards with zero
configuration or options.

While there is some redundancy with
[stm32h7xx-hal](https://github.com/stm32-rs/stm32h7xx-hal), the two projects
have different aims. Here, the goal is to provide an easy on-ramp to the usage of
these boards without covering all possible features or providing a universally helpful 
library. For more advanced usage, you are encouraged to take the lessons 
learned here and directly use the
[stm32h7xx-hal](https://github.com/stm32-rs/stm32h7xx-hal) crate. Of course, as
mentioned above, we welcome suggested improvements that fit within the project
aim.

To facilitate ease-of-use with both the Nucleo-H743ZI and Nucleo-H743ZI2 boards,
the code in this repository should work by default on both boards without
modification or configuration.

## Differences between the Nucleo H743ZI and H743ZI2 Boards

| Feature | NUCLEO-H743ZI | NUCLEO-H743ZI2 | notes |
|---------|---------------|----------------|--------|
| STLINK | V2-1, Cuttable PCB | V3E, Embedded on PCB | see [DB3171, Rev 14](https://www.st.com/resource/en/data_brief/nucleo-l496zg.pdf) |
| Board reference | [UM1974 User Manual STM32 Nucleo-144 boards (MB1137)](https://www.st.com/resource/en/user_manual/dm00244518-stm32-nucleo144-boards-mb1137-stmicroelectronics.pdf) | [UM2407 User Manual STM32H7 Nucleo-144 boards (MB1364)](https://www.st.com/resource/en/user_manual/dm00499160-stm32h7-nucleo144-boards-mb1364-stmicroelectronics.pdf) ||
| User LD2 | Blue, connected to PB7 | Yellow, connected to PE1 | |

The code in this repository should work on both boards without modification or
configuration.

## Pre-requisists 

You will need the following components installed before building the project.

```
$ rustup target add thumbv7em-none-eabihf
$ rustup component add llvm-tools-preview
$ cargo install cargo-binutils
```
This will install a new target for the Rust compiler (`ARM Cortex-M4F` / `Cortex-M7F`) and a Cargo plugin to call `binutils` directly. 

For debugging the program, you will need to install a compatible version of `gdb` for your system.

## Building and running

**TODO: Update for probe-run**

Build with:

```sh
cargo build --release
```

Convert to a .bin file:

```sh
cargo objcopy --release --bin blinky -- -O binary target/thumbv7em-none-eabihf/release/blinky.bin
cargo objcopy --release --bin serial -- -O binary target/thumbv7em-none-eabihf/release/serial.bin
```

Flash the device:

```sh
cp target/thumbv7em-none-eabihf/release/blinky.bin /path/to/NODE_H743ZI/
cp target/thumbv7em-none-eabihf/release/serial.bin /path/to/NODE_H743ZI/
```

## Debugging with Knurling (`probe-rs`)

We use the Knurling project to facilitate debugging. `probe-rs` can be used to
debug the device from a host computer and view log messages send using the
`defmt` infrastructure.

To see `defmt` messages, compile with the `DEFMT_LOG` environment variable
set appropriately. (By default, `defmt` will show only error level messages.)

Powershell (Windows)
```
$Env:DEFMT_LOG="trace"
```

Bash (Linux/macOS)
```
export DEFMT_LOG=trace
```

### Probe option 1: onboard STLINKv3

This is the easiest option and works with only a micro-USB cable to your device.
If `probe-run` returns with `Error: The firmware on the probe is outdated`, you
can update the STLINKv3 firmware on your Nucleo using a download from
[st.com](https://www.st.com/en/development-tools/stsw-link007.html).

### Probe option 2: using Raspberry Pi Pico as a CMSIS-DAP probe

Debugging can be performed with a Raspberry Pi Pico board running the
[DapperMime firmware](https://github.com/majbthrd/DapperMime) to function as an
inexpensive (but highly capable) CMSIS-DAP probe. In this configuration, the
STLINKv3 hardware built into the Nucleo will be bypassed.

To setup your Nucleo for this, perform the following steps.

1. Connect the following pins

| Signal | MIPI-10 debug connector (CN5) on NUCLEO-H743ZI2 | Raspberry Pi Pico |
|---------|---------------|----------------|
| SWDIO | Pin 2 | GP3 |
| GND | Pin 3 | GND |
| SWCLK | Pin 4 | GP2|

2. Hold the STLINKv3 in reset state by using jumper JP1.
3. Power the Nucleo board by USB charger connected to the STLINK micro USB
   connector CN1 and setting the jumper JP2 to the CHGR position (from the
   default STLINK position).

## License

[0-clause BSD license](LICENSE-0BSD.txt).
