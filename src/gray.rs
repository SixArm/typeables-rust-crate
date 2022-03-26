//! # Gray
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::gray::*;
//! let x = GrayAsStructF64(1.0);
//! ```
//!
//! https://wikipedia.org/wiki/Gray
//!
//! The gray is a unit of absorbed dose (of ionizing radiation).
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is Gy.

//// Gray

pub struct GrayAsStructI8(pub i8);
pub struct GrayAsStructI16(pub i16);
pub struct GrayAsStructI32(pub i32);
pub struct GrayAsStructI64(pub i64);
pub struct GrayAsStructI128(pub i128);
pub struct GrayAsStructISize(pub isize);
pub struct GrayAsStructU8(pub u8);
pub struct GrayAsStructU16(pub u16);
pub struct GrayAsStructU32(pub u32);
pub struct GrayAsStructU64(pub u64);
pub struct GrayAsStructU128(pub u128);
pub struct GrayAsStructUSize(pub usize);
pub struct GrayAsStructF32(pub f32);
pub struct GrayAsStructF64(pub f64);

pub type GrayAsTypeI8 = i8;
pub type GrayAsTypeI16 = i16;
pub type GrayAsTypeI32 = i32;
pub type GrayAsTypeI64 = i64;
pub type GrayAsTypeI128 = i128;
pub type GrayAsTypeISize = isize;
pub type GrayAsTypeU8 = u8;
pub type GrayAsTypeU16 = u16;
pub type GrayAsTypeU32 = u32;
pub type GrayAsTypeU64 = u64;
pub type GrayAsTypeU128 = u128;
pub type GrayAsTypeUSize = usize;
pub type GrayAsTypeF32 = f32;
pub type GrayAsTypeF64 = f64;
