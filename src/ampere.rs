//! # Ampere
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::ampere::*;
//! let x = AmpereAsStructF64(1.0);
//! ```
//! 
//! https://wikipedia.org/wiki/Ampere
//! 
//! The ampere, often shortened to amp, is the SI base unit of electric current.
//! The ampere is named after French mathematician and physicist André-Marie
//! Ampère (1775–1836), considered the father of electromagnetism along with the
//! Danish physicist Hans Christian Ørsted. 

//// Ampere

pub struct AmpereAsStructI8(pub i8);
pub struct AmpereAsStructI16(pub i16);
pub struct AmpereAsStructI32(pub i32);
pub struct AmpereAsStructI64(pub i64);
pub struct AmpereAsStructI128(pub i128);
pub struct AmpereAsStructISize(pub isize);
pub struct AmpereAsStructU8(pub u8);
pub struct AmpereAsStructU16(pub u16);
pub struct AmpereAsStructU32(pub u32);
pub struct AmpereAsStructU64(pub u64);
pub struct AmpereAsStructU128(pub u128);
pub struct AmpereAsStructUSize(pub usize);
pub struct AmpereAsStructF32(pub f32);
pub struct AmpereAsStructF64(pub f64);

pub type AmpereAsTypeI8 = i8;
pub type AmpereAsTypeI16 = i16;
pub type AmpereAsTypeI32 = i32;
pub type AmpereAsTypeI64 = i64;
pub type AmpereAsTypeI128 = i128;
pub type AmpereAsTypeISize = isize;
pub type AmpereAsTypeU8 = u8;
pub type AmpereAsTypeU16 = u16;
pub type AmpereAsTypeU32 = u32;
pub type AmpereAsTypeU64 = u64;
pub type AmpereAsTypeU128 = u128;
pub type AmpereAsTypeUSize = usize;
pub type AmpereAsTypeF32 = f32;
pub type AmpereAsTypeF64 = f64;
