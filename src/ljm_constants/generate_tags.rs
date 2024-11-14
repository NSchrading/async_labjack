use regex::Regex;
use serde::de::Deserializer;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

struct LabjackTag<'a> {
    name: &'a str,
    register_type: &'a str,
    readwrite: &'a str,
    address: u16,
}

impl<'a> LabjackTag<'a> {
    fn new(name: &'a str, register_type: &'a str, readwrite: &'a str, address: u16) -> Self {
        LabjackTag {
            name,
            register_type,
            readwrite,
            address,
        }
    }
}

impl fmt::Display for LabjackTag<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let register_type = match self.register_type {
            "UINT16" => "u16",
            "UINT32" => "u32",
            "UINT64" => "u64",
            "INT32" => "i32",
            "FLOAT32" => "f32",
            "BYTE" => "Bytes",
            "STRING" => "Bytes",
            _ => panic!("{}", self.register_type),
        };

        write!(
            f,
            "pub const {}: LabjackTag<{}, {}, {}> = LabjackTag::new({});",
            self.name,
            register_type,
            if self.readwrite.contains("R") {
                "CanRead"
            } else {
                "CannotRead"
            },
            if self.readwrite.contains("W") {
                "CanWrite"
            } else {
                "CannotWrite"
            },
            self.address
        )
    }
}

fn main() {
    let address_re = Regex::new(r"#\(0:(\d+)\)").unwrap();
    let file = File::open("src/ljm_constants/ljm_constants.json").unwrap();
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");
    let registers = json
        .get("registers")
        .expect("ljm_constants should have a 'registers' key")
        .as_array()
        .expect("The 'registers' key should be an array.");

    let mut lib_file = File::create("src/lib.rs").unwrap();
    writeln!(lib_file, "use crate::labjack_tag::labjack_tag::{{CanRead, CanWrite, CannotRead, CannotWrite, LabjackTag}};");
    writeln!(lib_file, "use bytes::Bytes;");
    writeln!(lib_file, "");
    writeln!(lib_file, "pub mod helpers;");
    writeln!(lib_file, "pub mod labjack_tag;");
    writeln!(lib_file, "pub mod modbus_feedback;");
    writeln!(lib_file, "");
    // I define these as LabjackTag<T, R, W>s because these are simple 2 byte structs vs
    // the larger WritableLabjackTag / ReadableLabjackTag enums. This means users need to use
    // .into() to convert to the enum when necessary, but it saves on overall program
    // space since these are consts.

    for register in registers {
        let base_address: u16 = register
            .get("address")
            .expect("Each register must have a base address.")
            .as_number()
            .expect("The register address must be a number")
            .as_u64()
            .expect("The register address must be parseable to a u64")
            as u16;
        let name = register
            .get("name")
            .expect("Each register must have a name")
            .as_str()
            .expect("Register name must be a string")
            .to_uppercase();
        let register_type = register
            .get("type")
            .expect("Each register must have a type")
            .as_str()
            .expect("Register type must be a string");
        let readwrite_spec = register
            .get("readwrite")
            .expect("Each register must have a readwrite specification")
            .as_str()
            .expect("Register readwrite must be a string");
        if let Some(caps) = address_re.captures(&name) {
            let num_tags: u16 = caps
                .get(1)
                .expect("There should be a capture group with an ending index")
                .as_str()
                .parse()
                .expect("The index should be a number from 0-255");
            for idx_offset in 0..num_tags + 1 {
                let specific_name = address_re.replace(&name, idx_offset.to_string());

                let num_registers = match register_type {
                    "UINT16" => 1,
                    "UINT32" => 2,
                    "UINT64" => 4,
                    "INT32" => 2,
                    "FLOAT32" => 2,
                    _ => panic!("Unknown register type: {}", register_type),
                };

                let labjack_tag = LabjackTag::new(
                    &specific_name,
                    register_type,
                    readwrite_spec,
                    base_address + (idx_offset * num_registers),
                );
                writeln!(lib_file, "{}", labjack_tag);
            }
        } else {
            let labjack_tag = LabjackTag::new(&name, register_type, readwrite_spec, base_address);
            writeln!(lib_file, "{}", labjack_tag);
        }

        // secret registers I could include
        // pub const MA_COMM_ID: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49600);
        // pub const MA_PKT_SIZE_ETH_502: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(49910);
    }
}
