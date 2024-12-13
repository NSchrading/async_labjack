# async_labjack

[![Crates.io](https://img.shields.io/crates/v/async_labjack.svg)](https://crates.io/crates/async_labjack)
[![Docs.rs](https://docs.rs/async_labjack/badge.svg)](https://docs.rs/async_labjack)
[![CI](https://github.com/nschrading/async_labjack/workflows/CI/badge.svg)](https://github.com/nschrading/async_labjack/actions)

`async_labjack` is a pure rust async library to communicate with labjack T-series devices. It is completely standalone and does not require [LJM](https://support.labjack.com/docs/ljm-library-overview).

It differentiates itself from `LJM` and other available labjack crates in the rust ecosystem in the following ways:

* Pure rust. This is not an FFI binding to `LJM`. Instead it uses the [direct modbus TCP](https://support.labjack.com/docs/protocol-details-direct-modbus-tcp) interface of the labjack.

* Asynchronous via the tokio runtime and the `tokio_modbus` crate.

* Does not require `LJM` installed on your system. As long as you can establish a tcp connection to your labjack, you can use this library to work with your labjack.

* Strongly-typed. All registers (tags) that are available on the labjack have types and read/write specifications in this library. The rust compiler will prevent issues where, for example, you may be attempting to read a write-only register or get a floating point value from a u32 register.

* TCP-only. This library does not support USB connections to the labjack.

## Dev Pre-requisites

To use the faster linker specified in `.cargo/config.toml`, install lld:

### Windows

```bash
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

### Linux

```bash
# Ubuntu
sudo apt-get install lld clang
# Arch
sudo pacman -S lld clang
```

### MacOS

```bash
brew install llvm
```

## Installation

### Cargo

* Install the rust toolchain in order to have cargo installed by following
  [this](https://www.rust-lang.org/tools/install) guide.
* run `cargo install async_labjack`

## License

Licensed under

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).
