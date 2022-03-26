//! # Pascal
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::pascal::*;
//! let x = PascalAsStructF64(1.0);
//! ```
//!
//! https://wikipedia.org/wiki/Pascal
//!
//! The pascal is a unit of pressure, stress.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is Pa.

//// Pascal

pub struct PascalAsStructI8(pub i8);
pub struct PascalAsStructI16(pub i16);
pub struct PascalAsStructI32(pub i32);
pub struct PascalAsStructI64(pub i64);
pub struct PascalAsStructI128(pub i128);
pub struct PascalAsStructISize(pub isize);
pub struct PascalAsStructU8(pub u8);
pub struct PascalAsStructU16(pub u16);
pub struct PascalAsStructU32(pub u32);
pub struct PascalAsStructU64(pub u64);
pub struct PascalAsStructU128(pub u128);
pub struct PascalAsStructUSize(pub usize);
pub struct PascalAsStructF32(pub f32);
pub struct PascalAsStructF64(pub f64);

pub type PascalAsTypeI8 = i8;
pub type PascalAsTypeI16 = i16;
pub type PascalAsTypeI32 = i32;
pub type PascalAsTypeI64 = i64;
pub type PascalAsTypeI128 = i128;
pub type PascalAsTypeISize = isize;
pub type PascalAsTypeU8 = u8;
pub type PascalAsTypeU16 = u16;
pub type PascalAsTypeU32 = u32;
pub type PascalAsTypeU64 = u64;
pub type PascalAsTypeU128 = u128;
pub type PascalAsTypeUSize = usize;
pub type PascalAsTypeF32 = f32;
pub type PascalAsTypeF64 = f64;
