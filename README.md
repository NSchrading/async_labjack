# async_labjack

[![Crates.io](https://img.shields.io/crates/v/async_labjack.svg)](https://crates.io/crates/async_labjack)
[![Docs.rs](https://docs.rs/async_labjack/badge.svg)](https://docs.rs/async_labjack)
[![CI](https://github.com/nschrading/async_labjack/workflows/CI/badge.svg)](https://github.com/nschrading/async_labjack/actions)

`async_labjack` is a pure rust async library to communicate with LabJack T-series devices. It is completely standalone and does not require [LJM](https://support.labjack.com/docs/ljm-library-overview).

It differentiates itself from `LJM` and other available labjack crates in the rust ecosystem in the following ways:

* Pure rust. This is not an FFI binding to `LJM`. Instead it uses the [direct modbus TCP](https://support.labjack.com/docs/protocol-details-direct-modbus-tcp) interface of the LabJack.

* Asynchronous!

* Does not require `LJM` installed on your system. As long as you can establish a tcp connection to your LabJack, you can use this library to work with your LabJack.

* Strongly-typed. All registers (tags) that are available on the LabJack have types and read/write specifications in this library. The rust compiler will prevent issues where, for example, you may be attempting to read a write-only register or get a floating point value from a u32 register. This will prevent issues at compile time, rather than waiting to get back errors from the LabJack response at runtime.

* TCP-only. This library does not support USB connections to the LabJack.

## Example

```rust
use async_labjack::client::LabjackClient;
use async_labjack::{TEST, TEST_FLOAT32, TEST_INT32};

// Change to the address of your labjack
let socket_addr = "192.168.42.100:502".parse().unwrap();

let mut client = LabjackClient::connect_with_timeout(socket_addr, Duration::from_millis(3000))
    .await
    .unwrap();

// Read a single LabJack tag.
// Ensure the test value is always 0x00112233
let value = TEST.read(&mut client).await.unwrap();
assert_eq!(value, 0x00112233);

// Read and write multiple tags at once.
let results = client
  .read_write_tags(
      &[
          TEST_FLOAT32.into(),
          TEST_INT32.into(),
      ],
      &[
          TEST_FLOAT32.into(),
          TEST_INT32.into(),
      ],
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

// Stream data
const NUM_SCANS: u32 = 300;
const NUM_TAGS: u32 = 4;
const TOTAL_SAMPLES_EXPECTED: u32 = NUM_SCANS * NUM_TAGS;
let new_stream_config = StreamConfigBuilder::default()
    .num_addresses(NUM_TAGS)
    .scan_rate(1000.0)
    .num_scans(NUM_SCANS)
    .auto_target(16)
    .build()
    .unwrap();

client
    .start_stream(
        &new_stream_config,
        vec![
            STREAM_DEBUG_GET_SELF_INDEX.into(),
            STREAM_DEBUG_GET_SELF_INDEX.into(),
            STREAM_DEBUG_GET_SELF_INDEX.into(),
            STREAM_DEBUG_GET_SELF_INDEX.into(),
        ],
    )
    .await
    .unwrap();

// See examples/ dir for more complete streaming examples and others

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
