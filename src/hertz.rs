//! # Hertz
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::hertz::*;
//! let x = HertzAsStructF64(1.0);
//! ```
//!
//! https://wikipedia.org/wiki/Hertz
//!
//! The hertz is a unit of frequency.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is Hz.

//// Hertz

pub struct HertzAsStructI8(pub i8);
pub struct HertzAsStructI16(pub i16);
pub struct HertzAsStructI32(pub i32);
pub struct HertzAsStructI64(pub i64);
pub struct HertzAsStructI128(pub i128);
pub struct HertzAsStructISize(pub isize);
pub struct HertzAsStructU8(pub u8);
pub struct HertzAsStructU16(pub u16);
pub struct HertzAsStructU32(pub u32);
pub struct HertzAsStructU64(pub u64);
pub struct HertzAsStructU128(pub u128);
pub struct HertzAsStructUSize(pub usize);
pub struct HertzAsStructF32(pub f32);
pub struct HertzAsStructF64(pub f64);

pub type HertzAsTypeI8 = i8;
pub type HertzAsTypeI16 = i16;
pub type HertzAsTypeI32 = i32;
pub type HertzAsTypeI64 = i64;
pub type HertzAsTypeI128 = i128;
pub type HertzAsTypeISize = isize;
pub type HertzAsTypeU8 = u8;
pub type HertzAsTypeU16 = u16;
pub type HertzAsTypeU32 = u32;
pub type HertzAsTypeU64 = u64;
pub type HertzAsTypeU128 = u128;
pub type HertzAsTypeUSize = usize;
pub type HertzAsTypeF32 = f32;
pub type HertzAsTypeF64 = f64;
