use tokio_labjack_lib::{AIN0, TEST, TEST_FLOAT32, TEST_INT32, TEST_UINT16, TEST_UINT32};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use tokio_modbus::prelude::*;

    let socket_addr = "192.168.42.100:502".parse().unwrap();

    let mut ctx = tcp::connect(socket_addr).await?;

    println!("Fetching the coupler ID");

    // // fetch the data, it is returned in big endian
    // let data: Vec<u16> = ctx.read_input_registers(0x2, 2).await??;
    // // Combine the two u16s into a single u32 in big endian
    // let combined_value = (u32::from(data[0]) << 16) | u32::from(data[1]);
    // // Convert the u32 to f32
    // let value = f32::from_bits(combined_value);

    let value = AIN0.read(&mut ctx).await;

    println!("The data is {value:?}");

    // let value = TEST.read(&mut ctx).await;

    // println!("TEST: {value:?}");

    let value = TEST_FLOAT32.read(&mut ctx).await;

    println!("TEST_FLOAT32: {value:?}");

    TEST_FLOAT32.write(&mut ctx, 3.14).await;

    let value = TEST_FLOAT32.read(&mut ctx).await;

    println!("TEST_FLOAT32: {value:?}");

    let value = TEST_INT32.read(&mut ctx).await;

    println!("TEST_INT32: {value:?}");

    let value = TEST_UINT16.read(&mut ctx).await;

    println!("TEST_UINT16: {value:?}");

    let value = TEST_UINT32.read(&mut ctx).await;

    println!("TEST_UINT32: {value:?}");

    println!("Disconnecting");
    ctx.disconnect().await?;

    Ok(())
}
