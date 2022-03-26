//! # Tesla
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::tesla::*;
//! let x = TeslaAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Tesla
//! 
//! The tesla is a unit of magnetic induction, magnetic flux density.
//! 
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is T.

//// Tesla

pub struct TeslaAsStructI8(pub i8);
pub struct TeslaAsStructI16(pub i16);
pub struct TeslaAsStructI32(pub i32);
pub struct TeslaAsStructI64(pub i64);
pub struct TeslaAsStructI128(pub i128);
pub struct TeslaAsStructISize(pub isize);
pub struct TeslaAsStructU8(pub u8);
pub struct TeslaAsStructU16(pub u16);
pub struct TeslaAsStructU32(pub u32);
pub struct TeslaAsStructU64(pub u64);
pub struct TeslaAsStructU128(pub u128);
pub struct TeslaAsStructUSize(pub usize);
pub struct TeslaAsStructF32(pub f32);
pub struct TeslaAsStructF64(pub f64);

pub type TeslaAsTypeI8 = i8;
pub type TeslaAsTypeI16 = i16;
pub type TeslaAsTypeI32 = i32;
pub type TeslaAsTypeI64 = i64;
pub type TeslaAsTypeI128 = i128;
pub type TeslaAsTypeISize = isize;
pub type TeslaAsTypeU8 = u8;
pub type TeslaAsTypeU16 = u16;
pub type TeslaAsTypeU32 = u32;
pub type TeslaAsTypeU64 = u64;
pub type TeslaAsTypeU128 = u128;
pub type TeslaAsTypeUSize = usize;
pub type TeslaAsTypeF32 = f32;
pub type TeslaAsTypeF64 = f64;
