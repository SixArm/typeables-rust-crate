//! # Lux
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::lux::*;
//! let x = LuxAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Lux
//! 
//! The lux is a unit of illuminance.
//! 
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is lx.

//// Lux

pub struct LuxAsStructI8(pub i8);
pub struct LuxAsStructI16(pub i16);
pub struct LuxAsStructI32(pub i32);
pub struct LuxAsStructI64(pub i64);
pub struct LuxAsStructI128(pub i128);
pub struct LuxAsStructISize(pub isize);
pub struct LuxAsStructU8(pub u8);
pub struct LuxAsStructU16(pub u16);
pub struct LuxAsStructU32(pub u32);
pub struct LuxAsStructU64(pub u64);
pub struct LuxAsStructU128(pub u128);
pub struct LuxAsStructUSize(pub usize);
pub struct LuxAsStructF32(pub f32);
pub struct LuxAsStructF64(pub f64);

pub type LuxAsTypeI8 = i8;
pub type LuxAsTypeI16 = i16;
pub type LuxAsTypeI32 = i32;
pub type LuxAsTypeI64 = i64;
pub type LuxAsTypeI128 = i128;
pub type LuxAsTypeISize = isize;
pub type LuxAsTypeU8 = u8;
pub type LuxAsTypeU16 = u16;
pub type LuxAsTypeU32 = u32;
pub type LuxAsTypeU64 = u64;
pub type LuxAsTypeU128 = u128;
pub type LuxAsTypeUSize = usize;
pub type LuxAsTypeF32 = f32;
pub type LuxAsTypeF64 = f64;
