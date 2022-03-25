//! # Meter
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::meter::*;
//! let x = MeterAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Meter
//! 
//! The meter is a unit of length.
//! 
//! It is the base unit of length in the International System of Units (SI). 
//! 
//! The SI unit symbol is m. 
//! 
//! The spelling is meter (American English) or metre (British English).

//// Meter

pub struct MeterAsStructI8(pub i8);
pub struct MeterAsStructI16(pub i16);
pub struct MeterAsStructI32(pub i32);
pub struct MeterAsStructI64(pub i64);
pub struct MeterAsStructI128(pub i128);
pub struct MeterAsStructISize(pub isize);
pub struct MeterAsStructU8(pub u8);
pub struct MeterAsStructU16(pub u16);
pub struct MeterAsStructU32(pub u32);
pub struct MeterAsStructU64(pub u64);
pub struct MeterAsStructU128(pub u128);
pub struct MeterAsStructUSize(pub usize);
pub struct MeterAsStructF32(pub f32);
pub struct MeterAsStructF64(pub f64);

pub type MeterAsTypeI8 = i8;
pub type MeterAsTypeI16 = i16;
pub type MeterAsTypeI32 = i32;
pub type MeterAsTypeI64 = i64;
pub type MeterAsTypeI128 = i128;
pub type MeterAsTypeISize = isize;
pub type MeterAsTypeU8 = u8;
pub type MeterAsTypeU16 = u16;
pub type MeterAsTypeU32 = u32;
pub type MeterAsTypeU64 = u64;
pub type MeterAsTypeU128 = u128;
pub type MeterAsTypeUSize = usize;
pub type MeterAsTypeF32 = f32;
pub type MeterAsTypeF64 = f64;
