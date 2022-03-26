//! # Volt
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::volt::*;
//! let x = VoltAsStructF64(1.0);
//! ```
//!
//! https://wikipedia.org/wiki/Volt
//!
//! The volt is a unit of voltage, electrical potential difference, electromotive force.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is V.

//// Volt

pub struct VoltAsStructI8(pub i8);
pub struct VoltAsStructI16(pub i16);
pub struct VoltAsStructI32(pub i32);
pub struct VoltAsStructI64(pub i64);
pub struct VoltAsStructI128(pub i128);
pub struct VoltAsStructISize(pub isize);
pub struct VoltAsStructU8(pub u8);
pub struct VoltAsStructU16(pub u16);
pub struct VoltAsStructU32(pub u32);
pub struct VoltAsStructU64(pub u64);
pub struct VoltAsStructU128(pub u128);
pub struct VoltAsStructUSize(pub usize);
pub struct VoltAsStructF32(pub f32);
pub struct VoltAsStructF64(pub f64);

pub type VoltAsTypeI8 = i8;
pub type VoltAsTypeI16 = i16;
pub type VoltAsTypeI32 = i32;
pub type VoltAsTypeI64 = i64;
pub type VoltAsTypeI128 = i128;
pub type VoltAsTypeISize = isize;
pub type VoltAsTypeU8 = u8;
pub type VoltAsTypeU16 = u16;
pub type VoltAsTypeU32 = u32;
pub type VoltAsTypeU64 = u64;
pub type VoltAsTypeU128 = u128;
pub type VoltAsTypeUSize = usize;
pub type VoltAsTypeF32 = f32;
pub type VoltAsTypeF64 = f64;
