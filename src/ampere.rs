//! # Ampere
//!
//! Examples:
//! 
//! ```rust
//! # use ::typeables::ampere::*;
//! let x = AmpereAsStructF64(1);
//! ```
//! 
//! https://wikipedia.org/wiki/Ampere
//! 
//! The ampere, often shortened to amp, is the SI base unit of electric current.
//! The ampere is named after French mathematician and physicist André-Marie
//! Ampère (1775–1836), considered the father of electromagnetism along with the
//! Danish physicist Hans Christian Ørsted. 

//// Ampere

pub struct AmpereStructI8(pub i8);
pub struct AmpereStructI16(pub i16);
pub struct AmpereStructI32(pub i32);
pub struct AmpereStructI64(pub i64);
pub struct AmpereStructI128(pub i128);
pub struct AmpereStructISize(pub isize);
pub struct AmpereStructU8(pub u8);
pub struct AmpereStructU16(pub u16);
pub struct AmpereStructU32(pub u32);
pub struct AmpereStructU64(pub u64);
pub struct AmpereStructU128(pub u128);
pub struct AmpereStructUSize(pub usize);
pub struct AmpereStructF32(pub f32);
pub struct AmpereStructF64(pub f64);

pub type AmpereTypeI8 = i8;
pub type AmpereTypeI16 = i16;
pub type AmpereTypeI32 = i32;
pub type AmpereTypeI64 = i64;
pub type AmpereTypeI128 = i128;
pub type AmpereTypeISize = isize;
pub type AmpereTypeU8 = u8;
pub type AmpereTypeU16 = u16;
pub type AmpereTypeU32 = u32;
pub type AmpereTypeU64 = u64;
pub type AmpereTypeU128 = u128;
pub type AmpereTypeUSize = usize;
pub type AmpereTypeF32 = f32;
pub type AmpereTypeF64 = f64;
