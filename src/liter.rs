//! # Liter
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::liter::*;
//! let x = LiterAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Liter
//! 
//! The liter is a metric unit of volume. It is equal to 1 cubic decimeter
//! (dm3), 1000 cubic centimeters (cm3) or 0.001 cubic meter (m3). A cubic
//! decimeter (or liter) occupies a volume of 10 cm × 10 cm × 10 cm (see figure)
//! and is thus equal to one-thousandth of a cubic meter. 
//! 
//! The spelling is liter (American English) or litre (British Englis).
//! 
//! The symbols are L, l, ℓ.

//// Liter

pub struct LiterAsStructI8(pub i8);
pub struct LiterAsStructI16(pub i16);
pub struct LiterAsStructI32(pub i32);
pub struct LiterAsStructI64(pub i64);
pub struct LiterAsStructI128(pub i128);
pub struct LiterAsStructISize(pub isize);
pub struct LiterAsStructU8(pub u8);
pub struct LiterAsStructU16(pub u16);
pub struct LiterAsStructU32(pub u32);
pub struct LiterAsStructU64(pub u64);
pub struct LiterAsStructU128(pub u128);
pub struct LiterAsStructUSize(pub usize);
pub struct LiterAsStructF32(pub f32);
pub struct LiterAsStructF64(pub f64);

pub type LiterAsTypeI8 = i8;
pub type LiterAsTypeI16 = i16;
pub type LiterAsTypeI32 = i32;
pub type LiterAsTypeI64 = i64;
pub type LiterAsTypeI128 = i128;
pub type LiterAsTypeISize = isize;
pub type LiterAsTypeU8 = u8;
pub type LiterAsTypeU16 = u16;
pub type LiterAsTypeU32 = u32;
pub type LiterAsTypeU64 = u64;
pub type LiterAsTypeU128 = u128;
pub type LiterAsTypeUSize = usize;
pub type LiterAsTypeF32 = f32;
pub type LiterAsTypeF64 = f64;
