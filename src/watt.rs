//! # Watt
//!
//! Examples:
//!
//! ```rust
//! # use typeables::watt::*;
//! let x = WattAsStructF64(1.0);
//! ```
//!
//! <https://wikipedia.org/wiki/Watt>
//!
//! The watt is a unit of power or radiant flux.
//!
//! It is a derived unit in the International System of Units (SI).
//
//! The SI unit symbol is W.
//!
//! The watt is defined as a derived unit of (in SI base units) 1 kg⋅m2⋅s−3 or 1
//! joule per second. It is used to quantify the rate of energy transfer.
//!
//! The watt is named after James Watt (1736–1819).

//// Watt

pub struct WattAsStructI8(pub i8);
pub struct WattAsStructI16(pub i16);
pub struct WattAsStructI32(pub i32);
pub struct WattAsStructI64(pub i64);
pub struct WattAsStructI128(pub i128);
pub struct WattAsStructISize(pub isize);
pub struct WattAsStructU8(pub u8);
pub struct WattAsStructU16(pub u16);
pub struct WattAsStructU32(pub u32);
pub struct WattAsStructU64(pub u64);
pub struct WattAsStructU128(pub u128);
pub struct WattAsStructUSize(pub usize);
pub struct WattAsStructF32(pub f32);
pub struct WattAsStructF64(pub f64);

pub type WattAsTypeI8 = i8;
pub type WattAsTypeI16 = i16;
pub type WattAsTypeI32 = i32;
pub type WattAsTypeI64 = i64;
pub type WattAsTypeI128 = i128;
pub type WattAsTypeISize = isize;
pub type WattAsTypeU8 = u8;
pub type WattAsTypeU16 = u16;
pub type WattAsTypeU32 = u32;
pub type WattAsTypeU64 = u64;
pub type WattAsTypeU128 = u128;
pub type WattAsTypeUSize = usize;
pub type WattAsTypeF32 = f32;
pub type WattAsTypeF64 = f64;
