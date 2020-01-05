# f469

Learning Rust hard(ware) way...

To run & debug:

`openocd` in one terminal window (install it if you don't have it)

`cargo run --example <example_name>` in another one

then in gdb:
```
(gdb) continue
(gdb) next
// or whatever
```

New / working / tested examples for the board: `hello`, `blinky`, `serialecho`, `hashes`.

# Original readme

> A template for building applications for ARM Cortex-M microcontrollers

This project is developed and maintained by the [Cortex-M team][team].

## Dependencies

To build embedded programs using this template you'll need:

- Rust 1.31, 1.30-beta, nightly-2018-09-13 or a newer toolchain. e.g. `rustup
  default beta`

- The `cargo generate` subcommand. [Installation
  instructions](https://github.com/ashleygwilliams/cargo-generate#installation).

- `rust-std` components (pre-compiled `core` crate) for the ARM Cortex-M
  targets. Run:

``` console
$ rustup target add thumbv6m-none-eabi thumbv7m-none-eabi thumbv7em-none-eabi thumbv7em-none-eabihf
```
