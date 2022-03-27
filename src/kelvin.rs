//! # Kelvin
//!
//! Examples:
//!
//! ```rust
//! # use ::typeables::kelvin::*;
//! let x = KelvinAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Kelvin>
//!
//! The kelvin is the SI base unit of temperature.
//!
//! The symbol is K.
//!
//! The kelvin is named after the Belfast-born and University of Glasgow based
//! engineer and physicist William Thomson, 1st Baron Kelvin (1824–1907). The
//! Kelvin scale is an absolute thermodynamic temperature scale, meaning it uses
//! absolute zero as its null point.

//// Kelvin

pub struct KelvinAsStructI8(pub i8);
pub struct KelvinAsStructI16(pub i16);
pub struct KelvinAsStructI32(pub i32);
pub struct KelvinAsStructI64(pub i64);
pub struct KelvinAsStructI128(pub i128);
pub struct KelvinAsStructISize(pub isize);
pub struct KelvinAsStructU8(pub u8);
pub struct KelvinAsStructU16(pub u16);
pub struct KelvinAsStructU32(pub u32);
pub struct KelvinAsStructU64(pub u64);
pub struct KelvinAsStructU128(pub u128);
pub struct KelvinAsStructUSize(pub usize);
pub struct KelvinAsStructF32(pub f32);
pub struct KelvinAsStructF64(pub f64);

pub type KelvinAsTypeI8 = i8;
pub type KelvinAsTypeI16 = i16;
pub type KelvinAsTypeI32 = i32;
pub type KelvinAsTypeI64 = i64;
pub type KelvinAsTypeI128 = i128;
pub type KelvinAsTypeISize = isize;
pub type KelvinAsTypeU8 = u8;
pub type KelvinAsTypeU16 = u16;
pub type KelvinAsTypeU32 = u32;
pub type KelvinAsTypeU64 = u64;
pub type KelvinAsTypeU128 = u128;
pub type KelvinAsTypeUSize = usize;
pub type KelvinAsTypeF32 = f32;
pub type KelvinAsTypeF64 = f64;
