[![build](https://github.com/astraw/nucleo-h743zi/actions/workflows/cargo.yml/badge.svg)](https://github.com/astraw/nucleo-h743zi/actions/workflows/cargo.yml)

# nucleo-h743zi

Examples programs for nucleo-h743zi board.

This uses the [stm32h7xx-hal](https://github.com/astraw/stm32h7xx-hal) crate.

Collaboration on this crate is highly welcome as
are pull requests!

## Building and running

Build with:

    cargo build --release

Convert to a .bin file:

    cargo objcopy --release --bin blinky -- -O binary target/thumbv7em-none-eabihf/release/blinky.bin
    cargo objcopy --release --bin serial -- -O binary target/thumbv7em-none-eabihf/release/serial.bin

Flash the device:

    cp target/thumbv7em-none-eabihf/release/blinky.bin /path/to/NODE_H743ZI/
    cp target/thumbv7em-none-eabihf/release/serial.bin /path/to/NODE_H743ZI/

## Debugging

Debugging can be performed with tools like `openocd`. A sample `openocd` configuration file
is provided.

1. Uncomment one of the runners specified in the `.cargo/config` file according to the used operating
   system and desired debugger.
2. Run `openocd` in the project root
3. Run `cargo run --bin blinky`

## License

[0-clause BSD license](LICENSE-0BSD.txt).
