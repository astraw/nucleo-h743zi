# nucleo-h743zi

Examples programs for nucleo-h743zi board.

This uses the [stm32h7xx-hal](https://github.com/astraw/stm32h7xx-hal) crate.

Collaboration on this crate is highly welcome as
are pull requests!

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

## ITM Output

Using the ITM requires some configuration of the SWO, SWTF, ITM and DBGMCU registers.
By default, this will be done inside the application, but you can also opt out and perform this
step externally (for example with a `*.gdb` file) by passing `--no-default-features` to the build
command.

The `itm` binary target shows a simple ITM usage example. In order to display the output
sent form the MCU, install `itmdump` first

```sh
cargo install itmdump
```

Create an empty `itm.txt` in the crate root and then run the following command

```sh
itmdump -F -f itm.txt
```

Assuming you have selected an appropriate runner like specified in the Debugging chapter,
you can run

```sh
cargo run --bin itm
```

and you should receive some output into the ITM file.

## License

[0-clause BSD license](LICENSE-0BSD.txt).
