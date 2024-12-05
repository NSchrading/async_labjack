use regex::Regex;
use std::fmt;
use std::fs::File;
use std::io::Write;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct LabjackTag {
    address: u16,
    name: String,
    register_type: String,
    readwrite: String,
    description: String,
}

impl LabjackTag {
    fn new(
        address: u16,
        name: String,
        register_type: String,
        readwrite: String,
        description: String,
    ) -> Self {
        LabjackTag {
            address,
            name,
            register_type,
            readwrite,
            description,
        }
    }
}

impl fmt::Display for LabjackTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let register_type = match self.register_type.as_ref() {
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
            "/// {}\npub const {}: LabjackTag<{}, {}, {}> = LabjackTag::new({});",
            self.description,
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

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct LabjackError {
    number: u16,
    name: String,
    description: String,
}

impl LabjackError {
    fn new(number: u16, name: String, description: String) -> Self {
        LabjackError {
            number,
            name,
            description,
        }
    }
}

impl fmt::Display for LabjackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.description.len() > 0 {
            write!(
                f,
                "\t\t/// {}\n\t\t#[error(\"{}\")]\n\t\t{} = {},",
                self.description, self.description, self.name, self.number
            )
        } else {
            write!(
                f,
                "\t\t#[error(\"{}\")]\n\t\t{} = {},",
                self.name, self.name, self.number
            )
        }
    }
}

fn screaming_snake_to_upper_camel_case(input: &str) -> String {
    let mut output = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c == '_' {
            capitalize_next = true;
        } else {
            if capitalize_next {
                output.push_str(&c.to_uppercase().to_string());
            } else {
                output.push_str(&c.to_lowercase().to_string());
            }
            capitalize_next = false;
        }
    }

    output
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
    let errors = json
        .get("errors")
        .expect("ljm_constants should have an 'errors' key")
        .as_array()
        .expect("The 'errors' key should be an array.");

    let mut lib_file = File::create("src/lib.rs").unwrap();

    let header = r#"
use crate::helpers::macros::back_to_enum;
use crate::labjack_tag::{{CanRead, CanWrite, CannotRead, CannotWrite, LabjackTag}};
use bytes::Bytes;
use thiserror::Error;

pub mod client;
pub mod helpers;
pub mod labjack_tag;
pub mod modbus_feedback;
pub mod prelude;

#[derive(Debug, Error)]
pub enum TokioLabjackError {
    #[error(transparent)]
    LabjackError(#[from] LabjackError),
    #[error(transparent)]
    TokioModbusExceptionCode(#[from] tokio_modbus::ExceptionCode),
    #[error("Unknown status code for enum: {0}")]
    UnknownStatusCode(u16),
    #[error(transparent)]
    TokioModbusError(#[from] tokio_modbus::Error),
    #[error(transparent)]
    TimeElapsed(#[from] tokio::time::error::Elapsed),
    #[error(transparent)]
    Utf8Error(#[from] std::str::Utf8Error),
    #[error(transparent)]
    ProcessStreamSendError(#[from] tokio::sync::mpsc::error::SendError<u16>),
    #[error("{0}")]
    Other(String),
}

/// Specialized [`std::result::Result`] type
pub type Result<T> = std::result::Result<T, TokioLabjackError>;
    "#;
    writeln!(lib_file, "{}", header).unwrap();

    // I define these as LabjackTag<T, R, W>s because these are simple 2 byte structs vs
    // the larger WritableLabjackTag / ReadableLabjackTag enums. This means users need to use
    // .into() to convert to the enum when necessary, but it saves on overall program
    // space since these are consts.

    let mut labjack_tags: Vec<LabjackTag> = Vec::new();

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
            .expect("Register type must be a string")
            .to_string();
        let readwrite_spec = register
            .get("readwrite")
            .expect("Each register must have a readwrite specification")
            .as_str()
            .expect("Register readwrite must be a string")
            .to_string();
        let description = register
            .get("description")
            .expect("Each register must have a description")
            .as_str()
            .expect("Register description must be a string")
            .to_string();
        if let Some(caps) = address_re.captures(&name) {
            let num_tags: u16 = caps
                .get(1)
                .expect("There should be a capture group with an ending index")
                .as_str()
                .parse()
                .expect("The index should be a number from 0-255");
            for idx_offset in 0..num_tags + 1 {
                let specific_name = address_re
                    .replace(&name, idx_offset.to_string())
                    .to_string();

                let num_registers = match register_type.as_ref() {
                    "UINT16" => 1,
                    "UINT32" => 2,
                    "UINT64" => 4,
                    "INT32" => 2,
                    "FLOAT32" => 2,
                    _ => panic!("Unknown register type: {}", register_type),
                };

                let labjack_tag = LabjackTag::new(
                    base_address + (idx_offset * num_registers),
                    specific_name,
                    register_type.clone(),
                    readwrite_spec.clone(),
                    description.clone(),
                );
                labjack_tags.push(labjack_tag);
            }
        } else {
            let labjack_tag = LabjackTag::new(
                base_address,
                name,
                register_type,
                readwrite_spec,
                description,
            );
            labjack_tags.push(labjack_tag);
        }

        // secret registers I could include
        // pub const MA_COMM_ID: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49600);
        // pub const MA_PKT_SIZE_ETH_502: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(49910);
    }

    let labjack_tag = LabjackTag::new(
        4018,
        "STREAM_DATATYPE".to_string(),
        "UINT32".to_string(),
        "RW".to_string(),
        "Must be written with the value 0 for streaming.".to_string(),
    );
    labjack_tags.push(labjack_tag);

    labjack_tags.sort();
    for labjack_tag in &labjack_tags {
        writeln!(lib_file, "{}", labjack_tag).unwrap();
    }

    let mut labjack_errors: Vec<LabjackError> = Vec::new();
    for error in errors {
        let error_name = screaming_snake_to_upper_camel_case(
            &error
                .get("string")
                .expect("Each register must have a name")
                .as_str()
                .expect("Error name must be a string")
                .to_string(),
        );

        // The LJME errors are errors that come from LJM, not from the
        // LabjackDevice itself.
        if error_name.starts_with("Ljme") {
            continue;
        }

        let error_num: u16 = error
            .get("error")
            .expect("Each register must have an error number.")
            .as_number()
            .expect("The error number must be a number")
            .as_u64()
            .expect("The error number must be parseable to a u64")
            as u16;

        let description = if let Some(desc) = error.get("description") {
            desc.as_str()
                .expect("Register description must be a string")
                .to_string()
        } else {
            "".into()
        }
        .replace("\"", "");

        let labjack_error = LabjackError::new(error_num, error_name, description);
        labjack_errors.push(labjack_error);
    }

    labjack_errors.sort();
    writeln!(lib_file).unwrap();

    let enum_wrap = r#"
back_to_enum! {
    /// Enum containing all labjack error codes. These can be obtained by reading the
    /// [`LAST_ERR_DETAIL`] tag
    #[derive(Debug, Error)]
    #[repr(u16)]
    pub enum LabjackError {
"#;
    write!(lib_file, "{}", enum_wrap).unwrap();
    for labjack_error in &labjack_errors {
        writeln!(lib_file, "{}", labjack_error).unwrap();
    }
    writeln!(lib_file, "\t}}").unwrap();
    writeln!(lib_file, "}}").unwrap();
}
