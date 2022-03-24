//! # Year
//!
//! Example type alias:
//!
//! ```rust
//! # use ::typeables::year::*;
//! let x: YearAsTypeI16 = 2022;
//! ```
//!
//! Example struct tuple:
//!
//! ```rust
//! # use ::typeables::year::*;
//! let x = YearAsStructI16(2022);
//! ```

pub type YearAsTypeI8 = i8;
pub type YearAsTypeI16 = i16;
pub type YearAsTypeI32 = i32;
pub type YearAsTypeI64 = i64;
pub type YearAsTypeI128 = i128;
pub type YearAsTypeISize = isize;
pub type YearAsTypeU8 = u8;
pub type YearAsTypeU16 = u16;
pub type YearAsTypeU32 = u32;
pub type YearAsTypeU64 = u64;
pub type YearAsTypeU128 = u128;
pub type YearAsTypeUSize = usize;
pub type YearAsTypeF32 = f32;
pub type YearAsTypeF64 = f64;

pub struct YearAsStructI8(pub i8);
pub struct YearAsStructI16(pub i16);
pub struct YearAsStructI32(pub i32);
pub struct YearAsStructI64(pub i64);
pub struct YearAsStructI128(pub i128);
pub struct YearAsStructISize(pub isize);
pub struct YearAsStructU8(pub u8);
pub struct YearAsStructU16(pub u16);
pub struct YearAsStructU32(pub u32);
pub struct YearAsStructU64(pub u64);
pub struct YearAsStructU128(pub u128);
pub struct YearAsStructUSize(pub usize);
pub struct YearAsStructF32(pub f32);
pub struct YearAsStructF64(pub f64);
