use crate::labjack_tag::labjack_tag::{CanRead, CanWrite, CannotRead, CannotWrite, LabjackTag};
use bytes::Bytes;

pub mod helpers;
pub mod labjack_tag;
pub mod modbus_feedback;

pub enum LabjackType {
    UINT16 = 0,
    UINT32 = 1,
    INT32 = 2,
    FLOAT32 = 3,
    LJM_STRING = 98,
    LJM_BYTE = 99,
}

// I define these as LabjackTag<T, R, W>s because these are simple 2 byte structs vs
// the larger WritableLabjackTag / ReadableLabjackTag enums. This means users need to use
// .into() to convert to the enum when necessary, but it saves on overall program
// space since these are consts.
pub const AIN0: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(0);
pub const AIN1: LabjackTag<f32, CanRead, CannotWrite> = LabjackTag::new(2);
pub const TEST: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(55100);
pub const TEST_UINT16: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(55110);
pub const TEST_UINT32: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(55120);
pub const TEST_INT32: LabjackTag<i32, CanRead, CanWrite> = LabjackTag::new(55122);
pub const TEST_FLOAT32: LabjackTag<f32, CanRead, CanWrite> = LabjackTag::new(55124);

pub const FILE_IO_DIR_CURRENT: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60601);
pub const FILE_IO_PATH_READ_LEN_BYTES: LabjackTag<u32, CanRead, CannotWrite> =
    LabjackTag::new(60642);
pub const FILE_IO_PATH_READ: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(60652);

pub const FILE_IO_PATH_WRITE_LEN_BYTES: LabjackTag<u32, CannotRead, CanWrite> =
    LabjackTag::new(60640);
pub const FILE_IO_PATH_WRITE: LabjackTag<Bytes, CannotRead, CanWrite> = LabjackTag::new(60650);
pub const FILE_IO_OPEN: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60620);
pub const FILE_IO_READ: LabjackTag<Bytes, CanRead, CannotWrite> = LabjackTag::new(60656);
pub const FILE_IO_SIZE_BYTES: LabjackTag<u32, CanRead, CannotWrite> = LabjackTag::new(60628);
pub const FILE_IO_CLOSE: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60621);

pub const FILE_IO_DIR_FIRST: LabjackTag<u16, CannotRead, CanWrite> = LabjackTag::new(60610);

pub const MA_COMM_ID: LabjackTag<u32, CanRead, CanWrite> = LabjackTag::new(49600);
pub const MA_PKT_SIZE_ETH_502: LabjackTag<u16, CanRead, CanWrite> = LabjackTag::new(49910);
