//! # Hour
//!
//! The hour is a unit of time.
//!
//! Example:
//!
//! ```rust
//! # use typeables::hour::*;
//! let x = HourAsStructI16(1); // One hour of time
//! ```

pub struct HourAsStructI8(pub i8);
pub struct HourAsStructI16(pub i16);
pub struct HourAsStructI32(pub i32);
pub struct HourAsStructI64(pub i64);
pub struct HourAsStructI128(pub i128);
pub struct HourAsStructISize(pub isize);
pub struct HourAsStructU8(pub u8);
pub struct HourAsStructU16(pub u16);
pub struct HourAsStructU32(pub u32);
pub struct HourAsStructU64(pub u64);
pub struct HourAsStructU128(pub u128);
pub struct HourAsStructUSize(pub usize);
pub struct HourAsStructF32(pub f32);
pub struct HourAsStructF64(pub f64);

pub type HourAsTypeI8 = i8;
pub type HourAsTypeI16 = i16;
pub type HourAsTypeI32 = i32;
pub type HourAsTypeI64 = i64;
pub type HourAsTypeI128 = i128;
pub type HourAsTypeISize = isize;
pub type HourAsTypeU8 = u8;
pub type HourAsTypeU16 = u16;
pub type HourAsTypeU32 = u32;
pub type HourAsTypeU64 = u64;
pub type HourAsTypeU128 = u128;
pub type HourAsTypeUSize = usize;
pub type HourAsTypeF32 = f32;
pub type HourAsTypeF64 = f64;
