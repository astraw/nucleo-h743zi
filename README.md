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
these boards without covering all possible features or providing a universally helpful library. For more advanced usage, you are encouraged to take the lessons
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
$ rustup target add thumbv7em-none-eabihf`
$ rustup component add llvm-tools-preview`
$ cargo install cargo-binutils`
```
This will install a new target for the Rust compiler (`ARM Cortex-M4F` / `Cortex-M7F`) and a Cargo plugin to call `binutils` directly. 

If you also want to debug the program while running on the target, you also need to install `gdb` for the right architecture.

## Building and running

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
## Debugging

Debugging can be performed with tools like `openocd`. A sample `openocd` configuration file
is provided.

1. Uncomment one of the runners specified in the `.cargo/config` file according to the used operating
   system and desired debugger.
2. Run `openocd` in the project root
3. Run `cargo run --bin blinky` or `cargo run --bin serial`

You can also perform debugging with a GUI using VS Code with the Cortex-Debug extension.
Some configuration files were provided in the `.vscode` folder.

## License

[0-clause BSD license](LICENSE-0BSD.txt).
