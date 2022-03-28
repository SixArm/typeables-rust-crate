//! # Week
//!
//! The week is a unit of time.
//!
//! Example:
//!
//! ```rust
//! # use typeables::week::*;
//! let x = WeekAsStructF64(1.0); // One week of time
//! ```

pub struct WeekAsStructI8(pub i8);
pub struct WeekAsStructI16(pub i16);
pub struct WeekAsStructI32(pub i32);
pub struct WeekAsStructI64(pub i64);
pub struct WeekAsStructI128(pub i128);
pub struct WeekAsStructISize(pub isize);
pub struct WeekAsStructU8(pub u8);
pub struct WeekAsStructU16(pub u16);
pub struct WeekAsStructU32(pub u32);
pub struct WeekAsStructU64(pub u64);
pub struct WeekAsStructU128(pub u128);
pub struct WeekAsStructUSize(pub usize);
pub struct WeekAsStructF32(pub f32);
pub struct WeekAsStructF64(pub f64);

pub type WeekAsTypeI8 = i8;
pub type WeekAsTypeI16 = i16;
pub type WeekAsTypeI32 = i32;
pub type WeekAsTypeI64 = i64;
pub type WeekAsTypeI128 = i128;
pub type WeekAsTypeISize = isize;
pub type WeekAsTypeU8 = u8;
pub type WeekAsTypeU16 = u16;
pub type WeekAsTypeU32 = u32;
pub type WeekAsTypeU64 = u64;
pub type WeekAsTypeU128 = u128;
pub type WeekAsTypeUSize = usize;
pub type WeekAsTypeF32 = f32;
pub type WeekAsTypeF64 = f64;
