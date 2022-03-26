//! # Kilogram
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::kilogram::*;
//! let x = KilogramAsStructF64(1.0);
//! ```
//!
//! https://wikipedia.org/wiki/Kilogram
//!
//! The kilogram is a unit of mass.
//!
//! It is a base unit in the International System of Units (SI).
//
//! The SI unit symbol is kg.

//// Kilogram

pub struct KilogramAsStructI8(pub i8);
pub struct KilogramAsStructI16(pub i16);
pub struct KilogramAsStructI32(pub i32);
pub struct KilogramAsStructI64(pub i64);
pub struct KilogramAsStructI128(pub i128);
pub struct KilogramAsStructISize(pub isize);
pub struct KilogramAsStructU8(pub u8);
pub struct KilogramAsStructU16(pub u16);
pub struct KilogramAsStructU32(pub u32);
pub struct KilogramAsStructU64(pub u64);
pub struct KilogramAsStructU128(pub u128);
pub struct KilogramAsStructUSize(pub usize);
pub struct KilogramAsStructF32(pub f32);
pub struct KilogramAsStructF64(pub f64);

pub type KilogramAsTypeI8 = i8;
pub type KilogramAsTypeI16 = i16;
pub type KilogramAsTypeI32 = i32;
pub type KilogramAsTypeI64 = i64;
pub type KilogramAsTypeI128 = i128;
pub type KilogramAsTypeISize = isize;
pub type KilogramAsTypeU8 = u8;
pub type KilogramAsTypeU16 = u16;
pub type KilogramAsTypeU32 = u32;
pub type KilogramAsTypeU64 = u64;
pub type KilogramAsTypeU128 = u128;
pub type KilogramAsTypeUSize = usize;
pub type KilogramAsTypeF32 = f32;
pub type KilogramAsTypeF64 = f64;
