# async_labjack

[![Crates.io](https://img.shields.io/crates/v/async_labjack.svg)](https://crates.io/crates/async_labjack)
[![Docs.rs](https://docs.rs/async_labjack/badge.svg)](https://docs.rs/async_labjack)
[![CI](https://github.com/nschrading/async_labjack/workflows/CI/badge.svg)](https://github.com/nschrading/async_labjack/actions)

`async_labjack` is a pure rust async library to communicate with LabJack T-series devices. It is completely standalone and does not require [LJM](https://support.labjack.com/docs/ljm-library-overview).

It differentiates itself from `LJM` and other available labjack crates in the rust ecosystem in the following ways:

* Pure rust. This is not an FFI binding to `LJM`. Instead it uses the [direct modbus TCP](https://support.labjack.com/docs/protocol-details-direct-modbus-tcp) interface of the LabJack.

* Does not require `LJM` installed on your system. As long as you can establish a tcp connection to your LabJack, you can use this library to work with your LabJack.

* Asynchronous.

* Strongly-typed. All registers (tags) that are available on the LabJack have types and read/write specifications in this library. The rust compiler will prevent issues where, for example, you may be attempting to read a write-only register or get a floating point value from a u32 register. This will prevent issues at compile time, rather than waiting to get back errors from the LabJack response at runtime.

* All labjack registers and error codes are defined as constants in this library. See [all_tags](src/labjack/all_tags.rs) and [errors](src/labjack/errors.rs). When a labjack error occurs, all functions in this library will attempt to return the descriptive error from the `LAST_ERR_DETAIL` register, which you can match on via the `LabjackErrorCode` enum.

* TCP-only. This library does not support USB connections to the LabJack.

## Example

Many complete examples, including for streaming, can be found in the [examples](examples/) directory.

```rust
use async_labjack::client::LabjackClient;
use async_labjack::{TEST, TEST_FLOAT32, TEST_INT32};

// Change to the address of your labjack
let socket_addr = "192.168.42.100:502".parse().unwrap();

let client = &mut LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
    .await
    .unwrap();

// Read a single LabJack tag.
// Ensure the test value is always 0x00112233
let value = TEST.read(client).await.unwrap();
assert_eq!(value, 0x00112233);

// Read and write multiple tags at once.
// This is done in a single modbus function call, and writes occur before reads,
// so you can write to tags and immediately get back the newly written values.
let results = client
  .read_write_tags(
      // The tags to read
      &[
          TEST_FLOAT32.into(),
          TEST_INT32.into(),
      ],
      // The tags to write to
      &[
          TEST_FLOAT32.into(),
          TEST_INT32.into(),
      ],
      // The values to write to the tags
      &[
          HydratedTagValue::F32(-98765.43),
          HydratedTagValue::I32(-987654),
      ],
  )
  .await
  .unwrap();

let float32_val: f32 = (&results[0]).try_into().unwrap();
assert_eq!(float32_val, -98765.43);

let int32_val: i32 = (&results[1]).try_into().unwrap();
assert_eq!(int32_val, -987654);

client.disconnect().await.unwrap();
```

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
   ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## Contribution

See [CONTRIBUTING.md](CONTRIBUTING.md).

## Alternative Crates
- [ljmrs](https://crates.io/crates/ljmrs): A rust crate with FFI bindings to LJM, with an API that matches closely with the original LJM API. 

- [ljm](https://crates.io/crates/ljm): A rust crate with FFI bindings to LJM, with a goal of being more idiomatic to Rust and ergonomic for users when compared to `ljmrs`.
