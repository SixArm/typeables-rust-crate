//! # Month
//!
//! The month unit is a unit of time.
//!
//! Example:
//!
//! ```rust
//! use typeables::month::*;
//! let x = MonthAsStructF64(1.0); // One month of time
//! ```

pub struct MonthAsStructI8(pub i8);
pub struct MonthAsStructI16(pub i16);
pub struct MonthAsStructI32(pub i32);
pub struct MonthAsStructI64(pub i64);
pub struct MonthAsStructI128(pub i128);
pub struct MonthAsStructISize(pub isize);
pub struct MonthAsStructU8(pub u8);
pub struct MonthAsStructU16(pub u16);
pub struct MonthAsStructU32(pub u32);
pub struct MonthAsStructU64(pub u64);
pub struct MonthAsStructU128(pub u128);
pub struct MonthAsStructUSize(pub usize);
pub struct MonthAsStructF32(pub f32);
pub struct MonthAsStructF64(pub f64);

pub type MonthAsTypeI8 = i8;
pub type MonthAsTypeI16 = i16;
pub type MonthAsTypeI32 = i32;
pub type MonthAsTypeI64 = i64;
pub type MonthAsTypeI128 = i128;
pub type MonthAsTypeISize = isize;
pub type MonthAsTypeU8 = u8;
pub type MonthAsTypeU16 = u16;
pub type MonthAsTypeU32 = u32;
pub type MonthAsTypeU64 = u64;
pub type MonthAsTypeU128 = u128;
pub type MonthAsTypeUSize = usize;
pub type MonthAsTypeF32 = f32;
pub type MonthAsTypeF64 = f64;
