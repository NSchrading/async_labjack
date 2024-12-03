use async_trait::async_trait;
use bytes::Bytes;
use core::future;
use core::future::Future;
use mockall::mock;
use std::borrow::Cow;
use std::io;
use std::pin::Pin;
use tokio::time::Duration;
use tokio_labjack_lib::client::{LabjackClient, LabjackInteractions, LabjackKind};
use tokio_labjack_lib::helpers::bit_manipulation::be_bytes_to_u16_array;
use tokio_labjack_lib::labjack_tag::{CanRead, CanWrite, CannotWrite, LabjackTag};
use tokio_labjack_lib::modbus_feedback::mbfb::ModbusFeedbackFrame;
use tokio_labjack_lib::modbus_feedback::MBFB_FUNCTION_CODE;
use tokio_modbus::client::{Client, Context};
use tokio_modbus::slave::SlaveContext;
use tokio_modbus::Slave;
use tokio_modbus::{Request, Response, Result};

mock! {
    #[derive(Debug)]
    pub ModbusClient {}

    // We have to define the lifetimes specifically due to mockall limitations
    // See https://github.com/asomers/mockall/issues/541#issuecomment-1870690737
    #[async_trait]
    impl Client for ModbusClient {
        fn call<'life0, 'life1, 'async_trait>(
            &'life0 mut self,
            request: Request<'life1>,
        ) -> Pin<
                Box<dyn Future<Output = Result<Response>> + Send + 'async_trait>
            >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {}

        async fn disconnect(&mut self) -> io::Result<()>;
    }
    impl SlaveContext for ModbusClient {
        fn set_slave(&mut self, slave: Slave);
    }
}

#[tokio::test]
async fn test_labjack_tag_read() {
    let mut mock_client = MockModbusClient::new();

    // Define the mock behavior for call
    mock_client
        .expect_call()
        // 4 register call
        .withf(|req| matches!(req, Request::ReadInputRegisters(0, 4)))
        .returning(|_| {
            Box::pin(future::ready(Ok(Ok(Response::ReadInputRegisters(vec![
                0x1234, 0x5678, 0x9ABC, 0xDEF0,
            ])))))
        });
    mock_client
        .expect_call()
        // 2 register call
        .withf(|req| matches!(req, Request::ReadInputRegisters(0, 2)))
        .returning(|_| {
            Box::pin(future::ready(Ok(Ok(Response::ReadInputRegisters(vec![
                0x1234, 0x5678,
            ])))))
        });
    mock_client
        .expect_call()
        // 1 register call
        .withf(|req| matches!(req, Request::ReadInputRegisters(0, 1)))
        .returning(|_| {
            Box::pin(future::ready(Ok(Ok(Response::ReadInputRegisters(vec![
                0x1234,
            ])))))
        });

    let context = Context::from(Box::new(mock_client) as Box<dyn Client>);
    let mut mock_labjack_client = LabjackClient {
        context,
        address: "127.0.0.1:502".parse().unwrap(),
        command_response_timeout: Duration::from_secs(1),
        labjack_kind: LabjackKind::T7,
    };

    let u64_tag: LabjackTag<u64, CanRead, CannotWrite> = LabjackTag::new(0);
    let u32_tag: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(0);
    let f32_tag: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(0);
    let i32_tag: LabjackTag<i32, CanRead, CannotWrite> = LabjackTag::new(0);
    let u16_tag: LabjackTag<u16, CanRead, CannotWrite> = LabjackTag::new(0);

    let result = u64_tag.read(&mut mock_labjack_client).await;
    assert_eq!(result.unwrap(), 0x123456789ABCDEF0);

    let result = u32_tag.read(&mut mock_labjack_client).await;
    assert_eq!(result.unwrap(), 0x12345678);

    let result = f32_tag.read(&mut mock_labjack_client).await;
    assert_eq!(
        result.unwrap(),
        f32::from_be_bytes([0x12, 0x34, 0x56, 0x78])
    );

    let result = i32_tag.read(&mut mock_labjack_client).await;
    assert_eq!(result.unwrap(), 0x12345678);

    let result = u16_tag.read(&mut mock_labjack_client).await;
    assert_eq!(result.unwrap(), 0x1234);
}

#[tokio::test]
async fn test_labjack_tag_write() {
    let mut mock_client = MockModbusClient::new();

    // There are no writeable u64s

    let u32_tag: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(1000);
    let test_u32_value = u32::MAX;
    let expected_u32_data = be_bytes_to_u16_array(test_u32_value.to_be_bytes());
    let expected_u32_num_registers = expected_u32_data.len() as u16;

    let f32_tag: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(100);
    let test_f32_value: f32 = 123.45;
    let expected_f32_data = be_bytes_to_u16_array(test_f32_value.to_be_bytes());
    let expected_f32_num_registers = expected_f32_data.len() as u16;

    let i32_tag: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(10);
    let test_i32_value = i32::MIN;
    let expected_i32_data = be_bytes_to_u16_array(test_i32_value.to_be_bytes());
    let expected_i32_num_registers = expected_i32_data.len() as u16;

    let u16_tag: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(1);
    let test_u16_value = u16::MAX;

    // u32 call
    mock_client
        .expect_call()
        .withf(move |req| match req {
            Request::WriteMultipleRegisters(addr, data) => {
                matches!(data, Cow::Borrowed(slice) if slice == &expected_u32_data)
                    && addr == &u32_tag.address
            }
            _ => false,
        })
        .returning(move |_| {
            Box::pin(future::ready(Ok(Ok(Response::WriteMultipleRegisters(
                u32_tag.address,
                expected_u32_num_registers,
            )))))
        });

    // f32 call
    mock_client
        .expect_call()
        .withf(move |req| match req {
            Request::WriteMultipleRegisters(addr, data) => {
                matches!(data, Cow::Borrowed(slice) if slice == &expected_f32_data)
                    && addr == &f32_tag.address
            }
            _ => false,
        })
        .returning(move |_| {
            Box::pin(future::ready(Ok(Ok(Response::WriteMultipleRegisters(
                f32_tag.address,
                expected_f32_num_registers,
            )))))
        });

    // i32 call
    mock_client
        .expect_call()
        .withf(move |req| match req {
            Request::WriteMultipleRegisters(addr, data) => {
                matches!(data, Cow::Borrowed(slice) if slice == &expected_i32_data)
                    && addr == &i32_tag.address
            }
            _ => false,
        })
        .returning(move |_| {
            Box::pin(future::ready(Ok(Ok(Response::WriteMultipleRegisters(
                i32_tag.address,
                expected_i32_num_registers,
            )))))
        });

    // u16 call
    mock_client
        .expect_call()
        .withf(move |req| match req {
            Request::WriteSingleRegister(addr, data) => {
                addr == &u16_tag.address && data == &test_u16_value
            }
            _ => false,
        })
        .returning(move |_| {
            Box::pin(future::ready(Ok(Ok(Response::WriteSingleRegister(
                u16_tag.address,
                test_u16_value,
            )))))
        });

    let context = Context::from(Box::new(mock_client) as Box<dyn Client>);
    let mut mock_labjack_client = LabjackClient {
        context,
        address: "127.0.0.1:502".parse().unwrap(),
        command_response_timeout: Duration::from_secs(1),
        labjack_kind: LabjackKind::T7,
    };

    let result = u32_tag
        .write(&mut mock_labjack_client, test_u32_value)
        .await;
    assert!(result.is_ok());

    let result = f32_tag
        .write(&mut mock_labjack_client, test_f32_value)
        .await;
    assert!(result.is_ok());

    let result = i32_tag
        .write(&mut mock_labjack_client, test_i32_value)
        .await;
    assert!(result.is_ok());

    let result = u16_tag
        .write(&mut mock_labjack_client, test_u16_value)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_read_write_frame_bytes() {
    let mut mock_client = MockModbusClient::new();

    let read_address = 0;
    let write_address = 1;
    let read_counts = 10;
    let write_counts = 2;
    let read_address_ref = &[read_address];
    let write_address_ref = &[write_address];
    let read_counts_ref = &[read_counts];
    let write_counts_ref = &[write_counts];

    let mut mbfb = ModbusFeedbackFrame::new(
        read_address_ref,
        write_address_ref,
        read_counts_ref,
        write_counts_ref,
        Bytes::from(vec![0xAB, 0xCD, 0xEF, 0xFE]),
    );
    let mbfb_bytes = mbfb.to_bytes_mut();
    let expected_bytes = mbfb_bytes.clone();

    let mock_returned_bytes = Bytes::from(vec![0x01, 0x10]);
    let expected_returned_bytes = mock_returned_bytes.clone();

    // custom bytes call
    mock_client
        .expect_call()
        .withf(move |req| match req {
            Request::Custom(function_code, bytes) => {
                matches!(bytes, Cow::Borrowed(bytes) if bytes == &expected_bytes)
                    && function_code == &MBFB_FUNCTION_CODE
            }
            _ => false,
        })
        .returning(move |_| {
            Box::pin(future::ready(Ok(Ok(Response::Custom(
                MBFB_FUNCTION_CODE,
                expected_returned_bytes.clone(),
            )))))
        });

    let context = Context::from(Box::new(mock_client) as Box<dyn Client>);
    let mut mock_labjack_client = LabjackClient {
        context,
        address: "127.0.0.1:502".parse().unwrap(),
        command_response_timeout: Duration::from_secs(1),
        labjack_kind: LabjackKind::T7,
    };
    let result = mock_labjack_client.read_write_frame_bytes(mbfb_bytes).await;
    assert_eq!(result.unwrap(), mock_returned_bytes.clone());
}
