//! # Farad
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::farad::*;
//! let x = FaradAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Farad
//! 
//! The farad is a unit of electrical capacitance.
//! 
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is F.

//// Farad

pub struct FaradAsStructI8(pub i8);
pub struct FaradAsStructI16(pub i16);
pub struct FaradAsStructI32(pub i32);
pub struct FaradAsStructI64(pub i64);
pub struct FaradAsStructI128(pub i128);
pub struct FaradAsStructISize(pub isize);
pub struct FaradAsStructU8(pub u8);
pub struct FaradAsStructU16(pub u16);
pub struct FaradAsStructU32(pub u32);
pub struct FaradAsStructU64(pub u64);
pub struct FaradAsStructU128(pub u128);
pub struct FaradAsStructUSize(pub usize);
pub struct FaradAsStructF32(pub f32);
pub struct FaradAsStructF64(pub f64);

pub type FaradAsTypeI8 = i8;
pub type FaradAsTypeI16 = i16;
pub type FaradAsTypeI32 = i32;
pub type FaradAsTypeI64 = i64;
pub type FaradAsTypeI128 = i128;
pub type FaradAsTypeISize = isize;
pub type FaradAsTypeU8 = u8;
pub type FaradAsTypeU16 = u16;
pub type FaradAsTypeU32 = u32;
pub type FaradAsTypeU64 = u64;
pub type FaradAsTypeU128 = u128;
pub type FaradAsTypeUSize = usize;
pub type FaradAsTypeF32 = f32;
pub type FaradAsTypeF64 = f64;
