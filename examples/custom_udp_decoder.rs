// udp_modbus_codec.rs
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;

use futures::{SinkExt, StreamExt};
use std::io::Cursor;
use std::{
    borrow::Cow,
    error,
    fmt::{self, Display},
};
use tokio::net::UdpSocket;
use tokio_util::codec::{Decoder, Encoder};
use tokio_util::udp::UdpFramed;
// These types are part of the `tokio-modbus` crate.
// Ensure you have `tokio-modbus` (e.g., version 0.8 or similar) in your Cargo.toml.
// The exact paths might vary slightly based on the `tokio-modbus` version and features.
use bytes::{BufMut, Bytes, BytesMut};

use tokio_modbus::prelude::{ExceptionCode, Request, Response}; // Byte manipulation utilities

const MBAP_HEADER_LEN: usize = 7; // Modbus Application Protocol Header length
const MODBUS_PROTOCOL_ID: u16 = 0x0000; // Standard for Modbus TCP, often reused for UDP
const MAX_PDU_SIZE: usize = 253;
const PROTOCOL_ID: u16 = 0x0000; // TCP
type Coil = bool;
use async_labjack::{
    ETHERNET_MAC, ETHERNET_UDP_DISCOVERY_ONLY_DEFAULT, LAST_ERR_DETAIL, LAST_ERR_TRANSACTION_ID,
    TEST, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32,
};

/// Decoder for the Modbus Application Data Unit (ADU) from UDP datagrams.
/// It parses the MBAP header and extracts the PDU.
#[derive(Debug, Default)]
struct UdpAduDecoder;

type TransactionId = u16;
type UnitId = u8;

/// Represents a message from the client (slave) to the server (master).
#[derive(Debug, Clone)]
struct RequestPdu<'a>(Request<'a>);

impl<'a> From<Request<'a>> for RequestPdu<'a> {
    fn from(from: Request<'a>) -> Self {
        RequestPdu(from)
    }
}

impl<'a> From<RequestPdu<'a>> for Request<'a> {
    fn from(from: RequestPdu<'a>) -> Self {
        from.0
    }
}

/// A Modbus function code.
///
/// All function codes as defined by the protocol specification V1.1b3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctionCode {
    /// 01 (0x01) Read Coils.
    ReadCoils,

    /// 02 (0x02) Read Discrete Inputs
    ReadDiscreteInputs,

    /// 03 (0x03) Read Holding Registers
    ReadHoldingRegisters,

    /// 04 (0x04) Read Input Registers
    ReadInputRegisters,

    /// 05 (0x05) Write Single Coil
    WriteSingleCoil,

    /// 06 (0x06) Write Single Register
    WriteSingleRegister,

    /// 07 (0x07) Read Exception Status (Serial Line only)
    ReadExceptionStatus,

    /// 08 (0x08) Diagnostics (Serial Line only)
    Diagnostics,

    /// 11 (0x0B) Get Comm Event Counter (Serial Line only)
    GetCommEventCounter,

    /// 12 (0x0C) Get Comm Event Log (Serial Line only)
    GetCommEventLog,

    /// 15 (0x0F) Write Multiple Coils
    WriteMultipleCoils,

    /// 16 (0x10) Write Multiple Registers
    WriteMultipleRegisters,

    /// 17 (0x11) Report Slave ID (Serial Line only)
    ReportServerId,

    /// 20 (0x14) Read File Record
    ReadFileRecord,

    /// 21 (0x15) Write File Record
    WriteFileRecord,

    /// 22 (0x16) Mask Write Register
    MaskWriteRegister,

    /// 23 (0x17) Read/Write Multiple Registers
    ReadWriteMultipleRegisters,

    /// 24 (0x18) Read FIFO Queue
    ReadFifoQueue,

    /// 43 ( 0x2B) Encapsulated Interface Transport
    EncapsulatedInterfaceTransport,

    /// Custom Modbus Function Code.
    Custom(u8),
}

impl FunctionCode {
    /// Create a new [`FunctionCode`] with `value`.
    #[must_use]
    pub const fn new(value: u8) -> Self {
        match value {
            0x01 => Self::ReadCoils,
            0x02 => Self::ReadDiscreteInputs,
            0x03 => Self::ReadHoldingRegisters,
            0x04 => Self::ReadInputRegisters,
            0x05 => Self::WriteSingleCoil,
            0x06 => Self::WriteSingleRegister,
            0x07 => Self::ReadExceptionStatus,
            0x08 => Self::Diagnostics,
            0x0B => Self::GetCommEventCounter,
            0x0C => Self::GetCommEventLog,
            0x0F => Self::WriteMultipleCoils,
            0x10 => Self::WriteMultipleRegisters,
            0x11 => Self::ReportServerId,
            0x14 => Self::ReadFileRecord,
            0x15 => Self::WriteFileRecord,
            0x16 => Self::MaskWriteRegister,
            0x17 => Self::ReadWriteMultipleRegisters,
            0x18 => Self::ReadFifoQueue,
            0x2B => Self::EncapsulatedInterfaceTransport,
            code => Self::Custom(code),
        }
    }

    /// Gets the [`u8`] value of the current [`FunctionCode`].
    #[must_use]
    pub const fn value(self) -> u8 {
        match self {
            Self::ReadCoils => 0x01,
            Self::ReadDiscreteInputs => 0x02,
            Self::ReadHoldingRegisters => 0x03,
            Self::ReadInputRegisters => 0x04,
            Self::WriteSingleCoil => 0x05,
            Self::WriteSingleRegister => 0x06,
            Self::ReadExceptionStatus => 0x07,
            Self::Diagnostics => 0x08,
            Self::GetCommEventCounter => 0x0B,
            Self::GetCommEventLog => 0x0C,
            Self::WriteMultipleCoils => 0x0F,
            Self::WriteMultipleRegisters => 0x10,
            Self::ReportServerId => 0x11,
            Self::ReadFileRecord => 0x14,
            Self::WriteFileRecord => 0x15,
            Self::MaskWriteRegister => 0x16,
            Self::ReadWriteMultipleRegisters => 0x17,
            Self::ReadFifoQueue => 0x18,
            Self::EncapsulatedInterfaceTransport => 0x2B,
            Self::Custom(code) => code,
        }
    }
}

impl Display for FunctionCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.value().fmt(f)
    }
}

/// A server (slave) exception response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ExceptionResponse {
    pub function: FunctionCode,
    pub exception: ExceptionCode,
}

/// Represents a message from the server (slave) to the client (master).
#[derive(Debug, Clone, PartialEq, Eq)]
struct ResponsePdu(Result<Response, ExceptionResponse>);

impl From<Response> for ResponsePdu {
    fn from(from: Response) -> Self {
        ResponsePdu(Ok(from))
    }
}

impl From<ExceptionResponse> for ResponsePdu {
    fn from(from: ExceptionResponse) -> Self {
        ResponsePdu(Err(from))
    }
}

impl From<ResponsePdu> for Result<Response, ExceptionResponse> {
    fn from(from: ResponsePdu) -> Self {
        from.0
    }
}

impl TryFrom<Bytes> for ExceptionResponse {
    type Error = Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let fn_err_code = bytes[0];
        if fn_err_code < 0x80 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Invalid exception function code",
            ));
        }
        let function = fn_err_code - 0x80;
        let exception = ExceptionCode::new(bytes[1]);
        Ok(ExceptionResponse {
            function: FunctionCode::new(function),
            exception,
        })
    }
}

impl TryFrom<Bytes> for ResponsePdu {
    type Error = Error;

    fn try_from(bytes: Bytes) -> Result<Self, Self::Error> {
        let fn_code = bytes[0];
        let pdu = if fn_code < 0x80 {
            Response::try_from(bytes)?.into()
        } else {
            ExceptionResponse::try_from(bytes)?.into()
        };
        Ok(pdu)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Header {
    transaction_id: TransactionId,
    unit_id: UnitId,
}

#[derive(Debug, Clone)]
struct RequestAdu<'a> {
    hdr: Header,
    pdu: RequestPdu<'a>,
}

#[derive(Debug, Clone)]
pub struct ResponseAdu {
    hdr: Header,
    pdu: ResponsePdu,
}

impl<'a> From<RequestAdu<'a>> for Request<'a> {
    fn from(from: RequestAdu<'a>) -> Self {
        from.pdu.into()
    }
}

impl Decoder for UdpAduDecoder {
    type Item = (Header, Bytes); // (MBAP Header, PDU Bytes)
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> std::io::Result<Option<(Header, Bytes)>> {
        tracing::debug!("in decode");
        println!("in decode");
        // For UDP, a single datagram is passed to decode.
        // If buf is empty, it means UdpFramed passed an empty datagram, which is unusual but possible.
        if buf.is_empty() {
            return Ok(None); // No data to process.
        }

        // 1. Check for minimum MBAP header length
        if buf.len() < MBAP_HEADER_LEN {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Datagram too short for MBAP header: got {} bytes, expected at least {}",
                    buf.len(),
                    MBAP_HEADER_LEN
                ),
            ));
        }

        // 2. Read the declared length from the MBAP header's length field.
        // This length field = Unit ID (1 byte) + PDU Length (n bytes).
        let mbap_declared_length_after_header = u16::from_be_bytes([buf[4], buf[5]]) as usize;
        if mbap_declared_length_after_header == 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "MBAP length field is 0, which is invalid (must include at least 1 byte for Unit ID)",
            ));
        }
        // PDU length = MBAP Length Field - 1 (for the Unit ID byte)
        let pdu_len = mbap_declared_length_after_header - 1;

        // 3. Check if the datagram's actual size matches the expected ADU size.
        let expected_adu_len = MBAP_HEADER_LEN + pdu_len;
        if buf.len() < expected_adu_len {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Datagram shorter than declared MBAP length: ADU has {} bytes, but header implies {} bytes (MBAP Header {} + PDU {})",
                    buf.len(),
                    expected_adu_len,
                    MBAP_HEADER_LEN,
                    pdu_len
                ),
            ));
        }
        if buf.len() > expected_adu_len {
            // Note: Some implementations might choose to truncate and process,
            // but for strictness, we'll consider it an error.
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Datagram longer than declared MBAP length: ADU has {} bytes, but header implies {} bytes. Extra {} bytes found.",
                    buf.len(),
                    expected_adu_len,
                    buf.len() - expected_adu_len
                ),
            ));
        }

        // 4. Split the header and PDU data.
        // `split_to` will panic if `MBAP_HEADER_LEN > buf.len()`, but we've checked this.
        let header_data = buf.split_to(MBAP_HEADER_LEN);

        // 5. Validate Protocol ID
        let protocol_id = u16::from_be_bytes([header_data[2], header_data[3]]);
        if protocol_id != MODBUS_PROTOCOL_ID {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Invalid protocol identifier: expected = {}, actual = {}",
                    MODBUS_PROTOCOL_ID, protocol_id
                ),
            ));
        }

        // 6. Extract header fields
        let transaction_id = u16::from_be_bytes([header_data[0], header_data[1]]);
        let unit_id = header_data[6];

        let header = Header {
            transaction_id,
            unit_id,
        };

        // The rest of the buffer is the PDU.
        // `split_to` will panic if `pdu_len > buf.len()`, but we've ensured `buf.len() == pdu_len` here.
        let pdu_data = buf.split_to(pdu_len).freeze();

        Ok(Some((header, pdu_data)))
    }
}

/// Codec for Modbus communication over UDP (Client Perspective).
/// Encodes `RequestAdu` and Decodes `ResponseAdu`.
#[derive(Debug)]
pub struct UdpClientCodec {
    adu_decoder: UdpAduDecoder,
}

impl UdpClientCodec {
    pub fn new() -> Self {
        Self {
            adu_decoder: UdpAduDecoder::default(),
        }
    }
}

impl Default for UdpClientCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder for UdpClientCodec {
    type Item = ResponseAdu;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> std::io::Result<Option<ResponseAdu>> {
        if let Some((hdr, pdu_data)) = self.adu_decoder.decode(buf)? {
            let pdu = ResponsePdu::try_from(pdu_data)?;
            Ok(Some(ResponseAdu { hdr, pdu }))
        } else {
            Ok(None)
        }
    }
}

fn packed_coils_size(coils: &[Coil]) -> usize {
    (coils.len() + 7) / 8
}

fn bool_to_coil(state: bool) -> u16 {
    if state {
        0xFF00
    } else {
        0x0000
    }
}

#[allow(clippy::cast_possible_truncation)]
fn u16_len(len: usize) -> u16 {
    // This type conversion should always be safe, because either
    // the caller is responsible to pass a valid usize or the
    // possible values are limited by the protocol.
    debug_assert!(len <= u16::MAX.into());
    len as u16
}

#[allow(clippy::cast_possible_truncation)]
fn u8_len(len: usize) -> u8 {
    // This type conversion should always be safe, because either
    // the caller is responsible to pass a valid usize or the
    // possible values are limited by the protocol.
    debug_assert!(len <= u8::MAX.into());
    len as u8
}

fn encode_packed_coils(buf: &mut bytes::BytesMut, coils: &[Coil]) -> usize {
    let packed_coils_size = packed_coils_size(coils);
    let offset = buf.len();
    buf.resize(offset + packed_coils_size, 0);
    let buf = &mut buf[offset..];
    for (i, b) in coils.iter().enumerate() {
        let v = u8::from(*b); // 0 or 1
        buf[i / 8] |= v << (i % 8);
    }
    packed_coils_size
}

fn encode_request_pdu(buf: &mut bytes::BytesMut, request: &Request<'_>) {
    tracing::debug!("in encode_request_pdu");
    println!("in encode_request_pdu");
    use tokio_modbus::{bytes::BufMut as _, Request::*};
    buf.put_u8(request.function_code().value());
    match request {
        ReadCoils(address, quantity)
        | ReadDiscreteInputs(address, quantity)
        | ReadInputRegisters(address, quantity)
        | ReadHoldingRegisters(address, quantity) => {
            buf.put_u16(*address);
            buf.put_u16(*quantity);
        }
        WriteSingleCoil(address, state) => {
            buf.put_u16(*address);
            buf.put_u16(bool_to_coil(*state));
        }
        WriteMultipleCoils(address, coils) => {
            buf.put_u16(*address);
            buf.put_u16(u16_len(coils.len()));
            buf.put_u8(u8_len(packed_coils_size(coils)));
            encode_packed_coils(buf, coils);
        }
        WriteSingleRegister(address, word) => {
            buf.put_u16(*address);
            buf.put_u16(*word);
        }
        WriteMultipleRegisters(address, words) => {
            buf.put_u16(*address);
            let len = words.len();
            buf.put_u16(u16_len(len));
            buf.put_u8(u8_len(len * 2));
            for w in words.as_ref() {
                buf.put_u16(*w);
            }
        }
        ReportServerId => {}
        MaskWriteRegister(address, and_mask, or_mask) => {
            buf.put_u16(*address);
            buf.put_u16(*and_mask);
            buf.put_u16(*or_mask);
        }
        ReadWriteMultipleRegisters(read_address, quantity, write_address, words) => {
            buf.put_u16(*read_address);
            buf.put_u16(*quantity);
            buf.put_u16(*write_address);
            let len = words.len();
            buf.put_u16(u16_len(len));
            buf.put_u8(u8_len(len * 2));
            for w in words.as_ref() {
                buf.put_u16(*w);
            }
        }
        Custom(_, custom_data) => {
            buf.put_slice(custom_data.as_ref());
        }
    }
    tracing::debug!("{:?}", &buf[..]);
    println!("{:?}", &buf[..]);
}

fn request_pdu_size(request: &Request<'_>) -> std::io::Result<usize> {
    use tokio_modbus::Request::*;
    let size = match request {
        ReadCoils(_, _)
        | ReadDiscreteInputs(_, _)
        | ReadInputRegisters(_, _)
        | ReadHoldingRegisters(_, _)
        | WriteSingleRegister(_, _)
        | WriteSingleCoil(_, _) => 5,
        WriteMultipleCoils(_, coils) => 6 + packed_coils_size(coils),
        WriteMultipleRegisters(_, data) => 6 + data.len() * 2,
        ReportServerId => 1,
        MaskWriteRegister(_, _, _) => 7,
        ReadWriteMultipleRegisters(_, _, _, data) => 10 + data.len() * 2,
        Custom(_, data) => 1 + data.len(),
    };
    if size > MAX_PDU_SIZE {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            "request PDU size exceeded",
        ));
    }
    Ok(size)
}

impl<'a> Encoder<RequestAdu<'a>> for UdpClientCodec {
    type Error = Error;

    fn encode(&mut self, adu: RequestAdu<'a>, buf: &mut BytesMut) -> std::io::Result<()> {
        tracing::debug!("in encode");
        println!("in encode");
        let RequestAdu {
            hdr,
            pdu: RequestPdu(request),
        } = adu;
        let request_pdu_size = request_pdu_size(&request)?;
        buf.reserve(request_pdu_size + 7);
        buf.put_u16(hdr.transaction_id);
        buf.put_u16(PROTOCOL_ID);
        buf.put_u16(u16_len(request_pdu_size + 1));
        buf.put_u8(hdr.unit_id);
        encode_request_pdu(buf, &request);
        println!("encoded bytes: {:?}", &buf[..]);
        tracing::debug!("encoded bytes: {:?}", &buf[..]);
        Ok(())
    }
}

// Example of how to use UdpClientCodec with UdpFramed
// To run this example, you'd need `tokio` and `futures` in your Cargo.toml
// in addition to `tokio-modbus`.
// e.g.,
// tokio = { version = "1", features = ["full"] }
// futures = "0.3"
// tokio-modbus = "0.8.1" # or your specific version
// byteorder = "1.4"
// tokio-util = { version = "0.7", features = ["codec"] }

#[allow(dead_code)] // To suppress warnings if this main is not directly run
async fn run_udp_modbus_client(
    device_ip: &str,
    device_port: u16,
    local_port: u16,
) -> std::io::Result<()> {
    let remote_addr_str = format!("{}:{}", device_ip, device_port);
    let remote_addr: SocketAddr = remote_addr_str.parse().expect("Invalid remote address");

    let local_addr_str = format!("0.0.0.0:{}", local_port);
    let local_addr: SocketAddr = local_addr_str.parse().expect("Invalid local address");

    println!("Binding to local address: {}", local_addr);
    println!("Attempting to connect to remote LabJack: {}", remote_addr);

    let socket = UdpSocket::bind(local_addr).await?;
    // For UdpFramed, you don't strictly need to `socket.connect()` if you send to specific addrs,
    // but `connect` sets a default remote and filters incoming packets to only that remote.
    // This is usually desired for a client-server interaction.
    socket.connect(remote_addr).await?;
    println!("UDP socket connected to {}", remote_addr);

    // Wrap the UDP socket with the UdpClientCodec
    let (mut framed_tx, mut framed_rx) = UdpFramed::new(socket, UdpClientCodec::new()).split();

    // --- Construct your custom Modbus request ---
    let transaction_id = 1; // Example: increment for each new transaction
    let unit_id = 255; // From your original code

    let mut custom_pdu_data = BytesMut::with_capacity(4);
    custom_pdu_data.put_u8(0);
    custom_pdu_data.put_u16(TEST_UINT32.address);
    custom_pdu_data.put_u8(2);
    let frozen_bytes = custom_pdu_data.freeze();

    // Use `Request::Custom` for non-standard function codes.
    // The first argument to `Request::Custom` is the function code.
    // The second is the data payload following the function code.
    let modbus_request_pdu = Request::Custom(76, Cow::Borrowed(&frozen_bytes));

    let request_adu = RequestAdu {
        hdr: Header {
            transaction_id,
            unit_id,
        },
        pdu: RequestPdu(modbus_request_pdu),
    };

    println!("Sending Modbus ADU: {:?}", request_adu);

    // Send the request
    if let Err(e) = framed_tx.send((request_adu, remote_addr)).await {
        eprintln!("Failed to send UDP packet: {}", e);
        return Err(e);
    }
    println!("Request sent successfully.");

    // Wait for a response
    println!("Waiting for response...");
    match tokio::time::timeout(std::time::Duration::from_secs(5), framed_rx.next()).await {
        Ok(Some(Ok((response_adu, _)))) => {
            println!("Received response ADU: {:?}", response_adu);
            // Process the ResponseAdu.
            // response_adu.pdu is ResponsePdu(Result<Response, Exception>)
            match response_adu.pdu.0 {
                Ok(Response::Custom(fc, data)) => {
                    println!("Parsed custom response: FC={}, Data={:?}", fc, &data[..]);
                    println!(
                        "{:?}",
                        u32::from_be_bytes([data[0], data[1], data[2], data[3]])
                    );
                    // Here you would parse `data` according to your LabJack's protocol for FC76.
                }
                Ok(other_response) => {
                    println!(
                        "Received unexpected standard Modbus response: {:?}",
                        other_response
                    );
                }
                Err(exception) => {
                    println!("Received Modbus exception: {:?}", exception);
                }
            }
        }
        Ok(Some(Err(e))) => {
            eprintln!("Error receiving response: {}", e);
            // This error could be from UdpAduDecoder (e.g., malformed packet)
            // or from UdpClientCodec's PDU parsing.
        }
        Ok(None) => {
            // This means the stream was closed, which is not expected in this client loop
            // unless the socket is closed elsewhere.
            eprintln!("UDP stream closed unexpectedly while waiting for response.");
        }
        Err(elapsed_err) => {
            // This is a timeout error from `tokio::time::timeout`.
            eprintln!("Timeout waiting for response: {}", elapsed_err);
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let labjack_ip = "192.168.42.100"; // Your LabJack's IP
    let labjack_udp_port = 52362; // Your LabJack's UDP port
    let local_bind_port = 0; // Port to bind locally, can be 0 for OS to pick

    // Note: Binding to the same port as the destination (52362) might be problematic
    // if the device expects responses to come from a different port or if you run multiple clients.
    // Using 0 for local_bind_port is often safer:
    // let local_bind_port = 0;

    if let Err(e) = run_udp_modbus_client(labjack_ip, labjack_udp_port, local_bind_port).await {
        eprintln!("Client error: {}", e);
    }
}
