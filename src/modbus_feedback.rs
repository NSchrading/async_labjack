//! Functions for constructing and interacting with the custom
//! [Labjack Modbus Feedback function](https://support.labjack.com/docs/protocol-details-direct-modbus-tcp#ProtocolDetails[DirectModbusTCP]-ModbusFeedback(MBFB,function#76))

/// The modbus function code the labjack uses internally to indicate the feedback function.
pub const MBFB_FUNCTION_CODE: u8 = 0x4C;

pub mod mbfb;
