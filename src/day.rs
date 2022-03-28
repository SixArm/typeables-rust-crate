//! # Day
//!
//! The day is a unit of time.
//!
//! Example:
//!
//! ```rust
//! # use typeables::day::*;
//! let x = DayAsStructF64(1.0); // One day of time
//! ```

pub struct DayAsStructI8(pub i8);
pub struct DayAsStructI16(pub i16);
pub struct DayAsStructI32(pub i32);
pub struct DayAsStructI64(pub i64);
pub struct DayAsStructI128(pub i128);
pub struct DayAsStructISize(pub isize);
pub struct DayAsStructU8(pub u8);
pub struct DayAsStructU16(pub u16);
pub struct DayAsStructU32(pub u32);
pub struct DayAsStructU64(pub u64);
pub struct DayAsStructU128(pub u128);
pub struct DayAsStructUSize(pub usize);
pub struct DayAsStructF32(pub f32);
pub struct DayAsStructF64(pub f64);

pub type DayAsTypeI8 = i8;
pub type DayAsTypeI16 = i16;
pub type DayAsTypeI32 = i32;
pub type DayAsTypeI64 = i64;
pub type DayAsTypeI128 = i128;
pub type DayAsTypeISize = isize;
pub type DayAsTypeU8 = u8;
pub type DayAsTypeU16 = u16;
pub type DayAsTypeU32 = u32;
pub type DayAsTypeU64 = u64;
pub type DayAsTypeU128 = u128;
pub type DayAsTypeUSize = usize;
pub type DayAsTypeF32 = f32;
pub type DayAsTypeF64 = f64;
