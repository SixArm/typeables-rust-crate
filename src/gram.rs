//! # Gram
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::gram::*;
//! let x = GramAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Gram
//! 
//! The gram is a unit of mass in the metric system.
//! 
//! The SI unit symbol is g.

//// Gram

pub struct GramAsStructI8(pub i8);
pub struct GramAsStructI16(pub i16);
pub struct GramAsStructI32(pub i32);
pub struct GramAsStructI64(pub i64);
pub struct GramAsStructI128(pub i128);
pub struct GramAsStructISize(pub isize);
pub struct GramAsStructU8(pub u8);
pub struct GramAsStructU16(pub u16);
pub struct GramAsStructU32(pub u32);
pub struct GramAsStructU64(pub u64);
pub struct GramAsStructU128(pub u128);
pub struct GramAsStructUSize(pub usize);
pub struct GramAsStructF32(pub f32);
pub struct GramAsStructF64(pub f64);

pub type GramAsTypeI8 = i8;
pub type GramAsTypeI16 = i16;
pub type GramAsTypeI32 = i32;
pub type GramAsTypeI64 = i64;
pub type GramAsTypeI128 = i128;
pub type GramAsTypeISize = isize;
pub type GramAsTypeU8 = u8;
pub type GramAsTypeU16 = u16;
pub type GramAsTypeU32 = u32;
pub type GramAsTypeU64 = u64;
pub type GramAsTypeU128 = u128;
pub type GramAsTypeUSize = usize;
pub type GramAsTypeF32 = f32;
pub type GramAsTypeF64 = f64;
