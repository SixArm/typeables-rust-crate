//! # Becquerel
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::becquerel::*;
//! let x = BecquerelAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Becquerel
//! 
//! The becquerel is a unit of radioactivity (decays per unit time).
//! 
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is Bq.

//// Becquerel

pub struct BecquerelAsStructI8(pub i8);
pub struct BecquerelAsStructI16(pub i16);
pub struct BecquerelAsStructI32(pub i32);
pub struct BecquerelAsStructI64(pub i64);
pub struct BecquerelAsStructI128(pub i128);
pub struct BecquerelAsStructISize(pub isize);
pub struct BecquerelAsStructU8(pub u8);
pub struct BecquerelAsStructU16(pub u16);
pub struct BecquerelAsStructU32(pub u32);
pub struct BecquerelAsStructU64(pub u64);
pub struct BecquerelAsStructU128(pub u128);
pub struct BecquerelAsStructUSize(pub usize);
pub struct BecquerelAsStructF32(pub f32);
pub struct BecquerelAsStructF64(pub f64);

pub type BecquerelAsTypeI8 = i8;
pub type BecquerelAsTypeI16 = i16;
pub type BecquerelAsTypeI32 = i32;
pub type BecquerelAsTypeI64 = i64;
pub type BecquerelAsTypeI128 = i128;
pub type BecquerelAsTypeISize = isize;
pub type BecquerelAsTypeU8 = u8;
pub type BecquerelAsTypeU16 = u16;
pub type BecquerelAsTypeU32 = u32;
pub type BecquerelAsTypeU64 = u64;
pub type BecquerelAsTypeU128 = u128;
pub type BecquerelAsTypeUSize = usize;
pub type BecquerelAsTypeF32 = f32;
pub type BecquerelAsTypeF64 = f64;
