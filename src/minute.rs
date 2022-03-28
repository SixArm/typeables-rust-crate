//! # Minute
//!
//! The minute is a unit of time.
//!
//! Example:
//!
//! ```rust
//! # use typeables::minute::*;
//! let x = MinuteAsStructI16(1); // One minute of time
//! ```

pub struct MinuteAsStructI8(pub i8);
pub struct MinuteAsStructI16(pub i16);
pub struct MinuteAsStructI32(pub i32);
pub struct MinuteAsStructI64(pub i64);
pub struct MinuteAsStructI128(pub i128);
pub struct MinuteAsStructISize(pub isize);
pub struct MinuteAsStructU8(pub u8);
pub struct MinuteAsStructU16(pub u16);
pub struct MinuteAsStructU32(pub u32);
pub struct MinuteAsStructU64(pub u64);
pub struct MinuteAsStructU128(pub u128);
pub struct MinuteAsStructUSize(pub usize);
pub struct MinuteAsStructF32(pub f32);
pub struct MinuteAsStructF64(pub f64);

pub type MinuteAsTypeI8 = i8;
pub type MinuteAsTypeI16 = i16;
pub type MinuteAsTypeI32 = i32;
pub type MinuteAsTypeI64 = i64;
pub type MinuteAsTypeI128 = i128;
pub type MinuteAsTypeISize = isize;
pub type MinuteAsTypeU8 = u8;
pub type MinuteAsTypeU16 = u16;
pub type MinuteAsTypeU32 = u32;
pub type MinuteAsTypeU64 = u64;
pub type MinuteAsTypeU128 = u128;
pub type MinuteAsTypeUSize = usize;
pub type MinuteAsTypeF32 = f32;
pub type MinuteAsTypeF64 = f64;
