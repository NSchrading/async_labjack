# tokio_labjack

[![Crates.io](https://img.shields.io/crates/v/tokio_labjack.svg)](https://crates.io/crates/tokio_labjack)
[![Docs.rs](https://docs.rs/tokio_labjack/badge.svg)](https://docs.rs/tokio_labjack)
[![CI](https://github.com/nschrading/tokio_labjack/workflows/CI/badge.svg)](https://github.com/nschrading/tokio_labjack/actions)

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
* run `cargo install tokio_labjack`

## License

Licensed under

 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).
